export const parse = {
  parseIds(json) {
    console.log("Parser is called");
    const data = JSON.parse(json);

    const ids = [];

    data.forEach((item) => {
      ids.push(Number(item.id));
    });

    console.log("Returning ids");
    console.log(ids);

    return ids;
  },
  parseTemperatures(json) {
    console.log("Parser is called");
    const data = JSON.parse(json);

    const temperatures = [];

    data.forEach((item) => {
      temperatures.push(item.temperature);
    });

    console.log("Returning temperatures");
    console.log(temperatures);

    return temperatures;
  },

  parseHumidities(json) {
    console.log("Parser is called");
    const data = JSON.parse(json);

    const humidities = [];

    data.forEach((item) => {
      humidities.push(item.humidity);
    });

    console.log("Returning humidities");
    console.log(humidities);

    return humidities;
  },
};
