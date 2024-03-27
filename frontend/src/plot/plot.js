// import { Plotly } from 'plotly.js-dist';
// import isOdd from 'is-odd';

export const plotFunctions = {
  hello: function hello(name) {
    return `Hello ${name}. Is 42 odd? ${isOdd(42)}`;
  },

  Dht11: class Dht11 {
    constructor(ids, temperatures, humidities) {
      this.ids = ids;
      this.temperatures = temperatures;
      this.humidities = humidities;
    }
  },

  buildDht11Data: function buildDht11Data(ids, temperatures, humidities) {
    return new this.Dht11Data(ids, temperatures, humidities);
  },

  plotLineChart: function plotLineChart(x, y, elementID) {
    const TESTER = document.getElementById(elementID);

    Plotly.newPlot(TESTER, [{
      x: x,
      y: y,
    }], {
      margin: { t: 0 },
    });
  }
};

