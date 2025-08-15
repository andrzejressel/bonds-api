import { TestBed } from '@angular/core/testing';
import { HttpClientTestingModule, HttpTestingController } from '@angular/common/http/testing';
import { BondDataService, BondData, BOND_API_URL } from './bond-data.service';

describe('BondDataService', () => {
  let service: BondDataService;
  let httpMock: HttpTestingController;
  const mockApiUrl = 'https://test-api.com';

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [HttpClientTestingModule],
      providers: [
        BondDataService,
        { provide: BOND_API_URL, useValue: mockApiUrl }
      ]
    });
    service = TestBed.inject(BondDataService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  describe('getBondNames', () => {
    it('should fetch bond names successfully', () => {
      const mockBondNames = ['Bond1', 'Bond2', 'Bond3'];

      service.getBondNames().subscribe(bondNames => {
        expect(bondNames).toEqual(mockBondNames);
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds`);
      expect(req.request.method).toBe('GET');
      req.flush(mockBondNames);
    });

    it('should handle HTTP error when fetching bond names', () => {
      service.getBondNames().subscribe({
        next: () => fail('Expected an error'),
        error: (error) => {
          expect(error).toBeTruthy();
        }
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds`);
      req.error(new ErrorEvent('Network error'));
    });
  });

  describe('getBondData', () => {
    const bondName = 'TestBond';
    const mockCsvData = `date,value
2023-01-01,100.5
2023-01-02,101.2
2023-01-03,99.8`;

    const expectedBondData: BondData = {
      dates: ['2023-01-01', '2023-01-02', '2023-01-03'],
      bondValues: [100.5, 101.2, 99.8]
    };

    it('should fetch and parse bond data successfully', (done) => {
      service.getBondData(bondName).subscribe(bondData => {
        expect(bondData).toEqual(expectedBondData);
        done();
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds/${bondName}/csv`);
      expect(req.request.method).toBe('GET');
      expect(req.request.responseType).toBe('text');
      req.flush(mockCsvData);
    });

    it('should handle empty CSV data', (done) => {
      const emptyCsvData = 'date,value';

      service.getBondData(bondName).subscribe(bondData => {
        expect(bondData.dates).toEqual([]);
        expect(bondData.bondValues).toEqual([]);
        done();
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds/${bondName}/csv`);
      req.flush(emptyCsvData);
    });

    it('should handle HTTP error and return user-friendly error message', (done) => {
      spyOn(console, 'error');

      service.getBondData(bondName).subscribe({
        next: () => fail('Expected an error'),
        error: (error) => {
          expect(error.message).toBe('Failed to load bond data. Please try again later.');
          expect(console.error).toHaveBeenCalledWith('Error fetching bond data:', jasmine.any(Object));
          done();
        }
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds/${bondName}/csv`);
      req.error(new ErrorEvent('Network error'));
    });

    it('should include timer delay in the response', (done) => {
      const startTime = Date.now();

      service.getBondData(bondName).subscribe(bondData => {
        const endTime = Date.now();
        const elapsed = endTime - startTime;

        expect(elapsed).toBeGreaterThanOrEqual(500);
        expect(bondData).toEqual(expectedBondData);
        done();
      });

      const req = httpMock.expectOne(`${mockApiUrl}/bonds/${bondName}/csv`);
      req.flush(mockCsvData);
    });
  });
});
