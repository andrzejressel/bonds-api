// Simple config loader that fetches API endpoint from public endpoint.json at runtime
import {Injectable} from '@angular/core';
import {environment} from '../../environments/environment';


@Injectable({providedIn: 'root'})
export class EndpointConfigService {
  private endpointUrl?: string;

  getEndpoint(): string {
    if (!this.endpointUrl) {
      throw new Error('EndpointConfigService: Endpoint URL not loaded yet. Call load() first.');
    }
    return this.endpointUrl;
  }

  async load(): Promise<void> {
    this.endpointUrl = await environment.getApiUrl();
  }
}

