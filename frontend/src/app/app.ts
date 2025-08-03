import {Component, OnDestroy, OnInit} from '@angular/core';
import {PlotlyModule} from 'angular-plotly.js';
import {MatToolbarModule} from '@angular/material/toolbar';
import {MatCardModule} from '@angular/material/card';
import {MatButtonModule} from '@angular/material/button';
import {MatIconModule} from '@angular/material/icon';
import {MatDividerModule} from '@angular/material/divider';
import {MatTabsModule} from '@angular/material/tabs';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatSelectModule} from '@angular/material/select';
import {FormControl, FormsModule, ReactiveFormsModule} from '@angular/forms';
import {CommonModule} from '@angular/common';
import {BondDataService, BondData} from './services/bond-data.service';
import {ReplaySubject, Subject, Subscription, takeUntil} from 'rxjs';
import {NgxMatSelectSearchModule} from 'ngx-mat-select-search';

@Component({
  selector: 'app-root',
  imports: [
    NgxMatSelectSearchModule,
    PlotlyModule,
    MatToolbarModule,
    MatCardModule,
    MatButtonModule,
    MatIconModule,
    MatDividerModule,
    MatTabsModule,
    MatProgressSpinnerModule,
    MatSelectModule,
    FormsModule,
    CommonModule,
    ReactiveFormsModule
  ],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App implements OnInit, OnDestroy {
  protected title = 'Obligacje Skarbowe';

  // Chart data for Polish government bonds
  public chartData: any[] = [];
  public chartLayout: any = {};
  public chartConfig: any = {};
  public bondNames: string[] = [];
  public filteredBanks: ReplaySubject<string[]> = new ReplaySubject<string[]>(1);

  // Loading and error states
  public isLoading = false;
  public errorMessage: string | null = null;

  public bankFilterCtrl: FormControl<string | null> = new FormControl<string>('');

  /** Subject that emits when the component has been destroyed. */
  protected _onDestroy = new Subject<void>();


  // Bond types
  public selectedBondType: string | null = null;

  private subscription: Subscription | null = null;

  constructor(private bondDataService: BondDataService) {
  }

  ngOnInit() {

    this.bondDataService.getBondNames().subscribe({
      next: (bondNames: string[]) => {
        this.bondNames = bondNames;
        this.selectedBondType = bondNames[0];
        this.filteredBanks.next(bondNames.slice())
        this.loadBondData();
      },
      error: (error: Error) => {
        this.errorMessage = error.message;
        this.isLoading = false;
        console.error('Error loading bond names:', error);
      }
    })

    this.bankFilterCtrl.valueChanges
      .pipe(takeUntil(this._onDestroy))
      .subscribe(() => {
        this.filterBanks();
      });

    this.loadBondData();
  }

  protected filterBanks() {
    if (!this.bondNames) {
      return;
    }
    // get the search keyword
    let search = this.bankFilterCtrl.value;
    if (!search) {
      this.filteredBanks.next(this.bondNames.slice());
      return;
    } else {
      search = search.toLowerCase();
    }
    // filter the banks
    this.filteredBanks.next(
      this.bondNames.filter(bank => bank.toLowerCase().indexOf(search) > -1)
    );
  }

  ngOnDestroy() {
    if (this.subscription) {
      this.subscription.unsubscribe();
    }
    this._onDestroy.next();
    this._onDestroy.complete();
  }

  /**
   * Loads bond data from the service for the selected bond type
   */
  public loadBondData(): void {
    this.isLoading = true;
    this.errorMessage = null;

    if (this.subscription) {
      this.subscription.unsubscribe();
    }

    if (this.selectedBondType === null) {
      return
    }

    this.subscription = this.bondDataService.getBondData(this.selectedBondType).subscribe({
      next: (data: BondData) => {
        console.log(data)
        this.setupBondChart(data);
        this.isLoading = false;
      },
      error: (error: Error) => {
        this.errorMessage = error.message;
        this.isLoading = false;
        console.error('Error loading bond data:', error);
      }
    });
  }

  /**
   * Handles bond type selection change
   */
  public onBondTypeChange(): void {
    this.loadBondData();
  }

  private setupBondChart(data: BondData) {
    // Use data from the service
    const {dates, bondValues} = data;

    // Get today's date object
    const today = new Date();

    // Format it as 'YYYY-MM-DD' to match the data format
    // padStart ensures we get '05' instead of '5' for the month/day
    const today_formatted = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;


    this.chartData = [
      {
        x: dates,
        y: bondValues,
        type: 'scatter',
        mode: "lines",
        name: `Obligacje`,
        // line: { color: '#FF6B6B', width: 3 },
        // marker: { size: 6 }
      },
      // {
      //   x: dates,
      //   y: bond5Years,
      //   type: 'scatter',
      //   mode: 'lines+markers',
      //   name: 'Obligacje 5-letnie',
      //   line: { color: '#4ECDC4', width: 3 },
      //   marker: { size: 6 }
      // },
      // {
      //   x: dates,
      //   y: bond10Years,
      //   type: 'scatter',
      //   mode: 'lines+markers',
      //   name: 'Obligacje 10-letnie',
      //   line: { color: '#45B7D1', width: 3 },
      //   marker: { size: 6 }
      // }
    ];

    this.chartLayout = {
      hovermode: "x",
      title: {
        text: `Rentowność Obligacji Skarbowych RP  (2024)`,
        font: {size: 24, color: '#2C3E50'}
      },
      xaxis: {
        title: 'Date',
        autorange: true,
        type: 'date'
      },
      yaxis: {
        title: 'Value',
        autorange: true
      },
      plot_bgcolor: '#FAFAFA',
      paper_bgcolor: 'white',
      font: {family: 'Inter, sans-serif'},
      // margin: { t: 80, r: 40, b: 80, l: 80 },
      // legend: {
      //   x: 0.02,
      //   y: 0.98,
      //   bgcolor: 'rgba(255,255,255,0.8)',
      //   bordercolor: '#CCCCCC',
      //   borderwidth: 1
      // }
      shapes: [
        {
          type: 'line',
          x0: today_formatted,  // Start x-coordinate
          x1: today_formatted,  // End x-coordinate (same for a vertical line)
          y0: 0,                // Start y-coordinate (bottom of chart)
          y1: 1,                // End y-coordinate (top of chart)
          yref: 'paper',        // Use 'paper' for y-coordinates relative to the plot area
          line: {
            color: 'red',
            width: 2,
            dash: 'dash'      // Style the line as dashed
          }
        }
      ],

      // NEW: Add an 'annotations' array for the "Today" label
      annotations: [
        {
          x: today_formatted,
          y: 1.05,             // Position it just above the top of the plot
          yref: 'paper',
          text: 'Today',
          showarrow: false,
          font: {
            color: 'red'
          }
        }
      ]
    };

    this.chartConfig = {
      responsive: true,
      displayModeBar: true,
      modeBarButtonsToRemove: ['pan2d', 'lasso2d', 'select2d'],
      displaylogo: false
    };
  }
}
