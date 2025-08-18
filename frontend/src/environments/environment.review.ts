export const environment = {
  getApiUrl: async () => {
    interface EndpointJson {
      endpoint?: string;
    }

    // Relative path resolves to /endpoint.json at the app root (served from assets)
    const response = await fetch('endpoint.json', {cache: 'no-store'});
    if (!response || !response.ok) {
      const status = response ? response.status : 'No response';
      const statusText = response ? response.statusText : '';
      throw new Error(`Failed to fetch endpoint.json (status: ${status}${statusText ? ', statusText: ' + statusText : ''})`);
    }
    const data = (await response.json()) as EndpointJson;
    const value = (data && typeof data.endpoint === 'string') ? data.endpoint.trim() : '';
    if (value) {
      return value;
    } else {
      throw Error("environment.review.ts: endpoint.json missing valid 'endpoint' value");
    }
  }
};
