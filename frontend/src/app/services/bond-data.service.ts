import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable, from, throwError, timer, forkJoin, first} from 'rxjs';
import {catchError, map} from 'rxjs/operators';
import * as d3 from 'd3';

export interface BondData {
  dates: string[];
  bondValues: number[];
}

@Injectable({
  providedIn: 'root'
})
export class BondDataService {
  constructor(private http: HttpClient) {
  }

  /**
   * Fetches bond data from the CSV file
   * @param bondType The type of bond to fetch data for
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

  /**
   * Fetches and parses the CSV file using d3.csv
   */
  private getCSVData(bondName: string): Observable<BondData> {
    // d3.csv returns a Promise that resolves to an array of objects
    // We convert it to an Observable using the 'from' operator
    const url = `assets/${bondName}.csv`;
    return from(d3.csv(url)).pipe(
      map(data => {
        const dates: string[] = [];
        const values: number[] = [];

        // Extract dates and values from the parsed CSV data
        data.forEach(row => {
          if (row['Date'] && row['Value']) {
            dates.push(row['Date']);
            values.push(+row['Value']); // Convert string to number using the + operator
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
