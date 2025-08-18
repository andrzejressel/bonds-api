import {
  ApplicationConfig,
  importProvidersFrom,
  provideBrowserGlobalErrorListeners,
  provideZoneChangeDetection,
  provideAppInitializer, inject
} from '@angular/core';
import { provideRouter } from '@angular/router';
declare var Plotly: any;
import { provideAnimations } from '@angular/platform-browser/animations';
import {provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';

import { routes } from './app.routes';
import {PlotlyModule} from 'angular-plotly.js';
import {BOND_API_URL} from './services/bond-data.service';
import { EndpointConfigService } from './services/endpoint-config.service';

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
    provideAppInitializer(() => {
      const cfg = inject(EndpointConfigService);
      return cfg.load()
    }),
    {
      provide: BOND_API_URL,
      useFactory: (cfg: EndpointConfigService) => cfg.getEndpoint(),
      deps: [EndpointConfigService]
    }
  ]
};
