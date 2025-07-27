import {TestBed} from '@angular/core/testing';
import {App} from './app';
import {BondDataService} from './services/bond-data.service';
import {BondFilesService} from './services/bond-files.service';
import {of} from 'rxjs';
import {NoopAnimationsModule} from '@angular/platform-browser/animations';
// @ts-ignore
import Plotly from 'plotly.js-dist';
import {PlotlyModule} from 'angular-plotly.js';

describe('App', () => {
  let component: App;
  let fixture: any;
  let mockBondDataService: jasmine.SpyObj<BondDataService>;
  let mockBondFilesService: jasmine.SpyObj<BondFilesService>;

  beforeEach(async () => {
    // Create mock services
    mockBondDataService = jasmine.createSpyObj('BondDataService', ['getBondData']);
    mockBondFilesService = jasmine.createSpyObj('BondFilesService', ['getBondNames']);

    // Setup mock return values
    mockBondFilesService.getBondNames.and.returnValue(of(['Bond1', 'Bond2', 'Bond3']));

    await TestBed.configureTestingModule({
      imports: [
        App,
        NoopAnimationsModule,
        PlotlyModule.forRoot(Plotly)
      ],
      providers: [
        { provide: BondDataService, useValue: mockBondDataService },
        { provide: BondFilesService, useValue: mockBondFilesService }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(App);
    component = fixture.componentInstance;
  });

  it('should create the app', () => {
    expect(component).toBeTruthy();
  });

});
