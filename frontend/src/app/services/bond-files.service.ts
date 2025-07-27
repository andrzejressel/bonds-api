import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class BondFilesService {
  private readonly filesJsonPath = 'assets/files.json';

  constructor(private http: HttpClient) {}

  getBondNames(): Observable<string[]> {
    return this.http.get<string[]>(this.filesJsonPath);
  }
}

