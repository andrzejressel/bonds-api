import {
  ApplicationConfig,
  importProvidersFrom,
  provideBrowserGlobalErrorListeners,
  provideZoneChangeDetection
} from '@angular/core';
import { provideRouter } from '@angular/router';
declare var Plotly: any;
import { provideAnimations } from '@angular/platform-browser/animations';
import {provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';

import { routes } from './app.routes';
import {PlotlyModule} from 'angular-plotly.js';
import {BOND_API_URL} from './services/bond-data.service';
import {environment} from '../environments/environment';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZoneChangeDetection({ eventCoalescing: true }),
    provideRouter(routes),
    provideAnimations(),
    provideHttpClient(withInterceptorsFromDi()),
    importProvidersFrom(
      PlotlyModule.forRoot(Plotly)
    ),
    {
      provide: BOND_API_URL,
      useValue: environment.apiUrl
    }
  ]
};
