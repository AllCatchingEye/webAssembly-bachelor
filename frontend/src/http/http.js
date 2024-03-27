export const httpHandle = {
  Request: class Request {
    constructor(uri, method, body = '') {
      this.uri = uri;
      this.method = method;
      this.body = body;
    }
  },

  buildRequest: function buildRequest(uri, method, body = '') {
    return new this.Request(uri, method, body);
  },

  Response: class Response {
    constructor(status, body) {
      this.status = status;
      this.body = body;
    }
  },

  fetchData: async function fetchData(req) {
    return fetch(req.uri, {
      method: req.method,
      body: req.body
    })
      .then((response) => {
        res = new Response(response.status, response.json());
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return res;
      });
  },
}
