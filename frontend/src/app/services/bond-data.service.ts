import {Inject, Injectable, InjectionToken} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable, from, throwError, timer, forkJoin, first, pipe} from 'rxjs';
import {catchError, map} from 'rxjs/operators';
import * as d3 from 'd3';

export interface BondData {
  dates: string[];
  bondValues: number[];
}

export const BOND_API_URL = new InjectionToken<string>('BOND_API_URL');

@Injectable({
  providedIn: 'root'
})
export class BondDataService {
  constructor(
    private http: HttpClient,
    @Inject(BOND_API_URL) private bondApiUrl: string
  ) {
  }

  getBondNames(): Observable<string[]> {
    return this.http.get<string[]>(this.bondApiUrl + "/bonds");
  }

  /**
   * Fetches bond data from the CSV file
   * @param bondName The name of bond to fetch data for
   * @returns Observable with bond data
   */
  getBondData(bondName: string): Observable<BondData> {
    const bondData = this.getCSVData(bondName).pipe(
      catchError(error => {
        console.error('Error fetching bond data:', error);
        return throwError(() => new Error('Failed to load bond data. Please try again later.'));
      })
    );


    return forkJoin([
        bondData,
        timer(500)
      ],
      (data, _) => data
    )
      .pipe(first())
  }


  private getCSVData(bondName: string): Observable<BondData> {

    return this.http.get(this.bondApiUrl + "/bonds/" + bondName + "/csv", {
      responseType: "text"
    })
      .pipe(
        map(csvContent => {
          const data = d3.csvParse(csvContent)
          console.log("csv", data)
          const dates: string[] = [];
          const values: number[] = [];

          // Extract dates and values from the parsed CSV data
          data.forEach(row => {
            if (row['date'] && row['value']) {
              dates.push(row['date']);
              values.push(+row['value']); // Convert string to number using the + operator
            }
          });

          return {
            dates: dates,
            bondValues: values,
          };
        })
      );
  }

}
