export namespace LocalPlotPlotFunctions {
  export { Dht11 };
  export function buildDht11Data(ids: Int32Array, temperatures: Int32Array, humidities: Int32Array): Dht11;
  export function plotLineChart(x: number, y: number, elementId: string): void;
  export function hello(name: string): string;
}

export class Dht11 {
  constructor(ids: Int32Array, temperatures: Int32Array, humidities: Int32Array)
}
