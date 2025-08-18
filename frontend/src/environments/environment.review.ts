export const environment = {
  getApiUrl: async () => {
    interface EndpointJson {
      endpoint?: string;
    }

    // Relative path resolves to /endpoint.json at the app root (served from assets)
    const response = await fetch('endpoint.json', {cache: 'no-store'});
    if (!response || !response.ok) {
      throw new Error('Failed to fetch endpoint.json');
    }
    const data = (await response.json()) as EndpointJson;
    const value = (data && typeof data.endpoint === 'string') ? data.endpoint.trim() : '';
    if (value) {
      return value;
    } else {
      throw Error("EndpointConfigService: endpoint.json missing valid 'endpoint' value");
    }
  }
};
