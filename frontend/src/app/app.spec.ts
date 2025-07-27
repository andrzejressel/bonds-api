import {TestBed} from '@angular/core/testing';
import {App} from './app';
import {importProvidersFrom} from '@angular/core';
import {PlotlyModule} from 'angular-plotly.js';
// @ts-ignore
import Plotly from 'plotly.js-dist'

// describe('App', () => {
//   beforeEach(async () => {
//     await TestBed.configureTestingModule({
//       imports: [
//         App,
//         importProvidersFrom(
//           PlotlyModule.forRoot(Plotly)
//         )
//       ],
//     }).compileComponents();
//   });
//
//   it('should create the app', () => {
//     const fixture = TestBed.createComponent(App);
//     const app = fixture.componentInstance;
//     expect(app).toBeTruthy();
//   });
//
//   it('should render title', () => {
//     const fixture = TestBed.createComponent(App);
//     fixture.detectChanges();
//     const compiled = fixture.nativeElement as HTMLElement;
//     expect(compiled.querySelector('h1')?.textContent).toContain('Hello, ObligacjeStarboweWebsite');
//   });
// });
