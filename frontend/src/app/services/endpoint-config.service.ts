// Simple config loader that fetches API endpoint from public endpoint.json at runtime
import {Injectable} from '@angular/core';
import {environment} from '../../environments/environment';


@Injectable({providedIn: 'root'})
export class EndpointConfigService {
  private endpointUrl?: string;

  getEndpoint(): string | undefined {
    return this.endpointUrl;
  }

  async load(): Promise<void> {
    this.endpointUrl = await environment.getApiUrl()
  }
}

