# webAssembly-bachelor

## Requirements

### General

Wasm-tools needs to be installed: https://github.com/bytecodealliance/wasm-tools

WAC needs to be installed to compose components together: https://github.com/bytecodealliance/wac

### Esp-idf

#### WebAssembly Micro Runtime

To run the esp the following requirements must be fulfilled:

* The Wasm Micro Runtime: https://github.com/bytecodealliance/wasm-micro-runtime

The path to WAMR should be exported

```export WAMR_PATH="/opt/wasm-micro-runtime/"```

#### Esp-Idf

* ESP IDF: https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/index.html

Then the following environment variables must be set:

```
export IDF_PATH="$HOME/esp/esp-idf"
alias get_idf='. $HOME/esp/esp-idf/export.sh'
```

#### Wasi-SDK

* The Wasi SDK must be installed: https://github.com/WebAssembly/wasi-sdk

The Paths for the WASI SDK may need to be adjusted in the following files:

* ./esp-idf/main/monitor_module/CMakeLists.txt
* ./esp-idf/main/monitor_component/CMakeLists.txt

I have also set the Following environment variables for the WASI SDK:

```
# wasi-sdk
export WASI_VERSION=22
export WASI_VERSION_FULL=${WASI_VERSION}.0
export WASI_SDK_PATH=/opt/wasi-sdk-${WASI_VERSION_FULL}
export WASI_TOOLCHAIN_FILE=${WASI_SDK_PATH}/share/cmake/wasi-sdk.cmake
export WASI_SYS_ROOT=${WASI_SDK_PATH}/share/wasi-sysroot/
alias CC="${WASI_SDK_PATH}/bin/clang"
```

In ```./esp-idf/build_and_run.sh``` the port of the line idf.py -p /dev/ttyUSB0 monitor may need to be adjusted

In ```./esp-idf/main/src/sensors.c``` in initilize_sensors() the Pin may need to be adjusted for the Dht11 sensor

### Frontend

I've created a virtual environment in ./frontend and installed the following python libraries with pip

* componentize-py
* wasmtime
* plotly

For the parser component written in Go:

* Go 
* TinyGo: https://tinygo.org/docs/guides/webassembly/wasi/

For the parser written in Javascript:

* Jco and componentize-js:

```
npm install @bytecodealliance/jco
npm install @bytecodealliance/componentize-js
```

### Adjusting the IP Address

I have used a static Ip address for the backend server. To adjust this Ip address to another address the following files have to be adjusted:

```
./backend/guest/src/lib.rs
./esp-idf/main/src/esp_client.c
./frontend/webserver/app.py
```

## Running the example

### Starting the backend Server

In order for the board and frontend to be able to send and request data from the backend, it should be started first, by running 

```./backend/build_and_run.sh```

### Running the Esp32 Board

To start the esp-idf simply run 

```./esp-idf/build_and_run.sh esp32```

To start the esp-idf simply run 

```./frontend/build_and_run.sh```

### Starting the Frontend webserver

## Exposee:

### Problemstellung

Das Thema dieser Bachelorarbeit befasst sich mit WebAssembly, wobei der Schwerpunkt auf der Entwicklung und Implementierung von Interfaces für Sensoren liegt. Dabei wird die WebAssembly Micro Runtime (WASMR) als Grundlage verwendet, die eine umfangreiche Sammlung von Tools und Bibliotheken bereitstellt, um WebAssembly in verschiedene Projekte zu integrieren und umzusetzen.

Die Interfaces in WebAssembly werden durch das Component-Modell implementiert. Dieses Modell stellt das Wasm Interface Type (WIT) Format zur Verfügung, mit dem Module erstellt werden können. In diesen Modulen werden Typen und Interfaces textuell beschrieben. Die definierten Typen und Schnittstellen können zwischen verschiedenen Modulen exportiert und importiert werden, was die Erstellung von Bibliotheken ermöglicht. Diese Module können dann für verschiedene Programmiersprachen generiert werden.

### Vorgehensweise

In der Bachelorarbeit wird ein Projekt erstellt, das aus verschiedenen Modulen besteht, die miteinander kommunizieren. Die Kommunikation zwischen den Modulen erfolgt mittels der Erstellung von Interfaces im WIT-Format. Die Interfaces werden definiert und sollen von den Modulen umgesetzt werden.

Die Verwendung von Interfaces wird auch im Embedded-Bereich getestet, zum Beispiel, indem der Zugriff auf eine API einer geeigneten Plattform ebenfalls mit einer Interfaces beschrieben wird. Falls nötig, muss hierfür noch ein zusätzlicher Wrapper implementiert werden, da das generierte Format der Interfaces möglicherweise nicht mit den Namen der APIs auf der Plattform übereinstimmt.

Die erstellten Interfaces werden praktisch getestet, indem das Projekt auf einer geeigneten Plattform deployed wird und die Kommunikation zwischen den Modoulen durch die Umsetzung der Interfaces beobachtet und getestet wird. Fehler werden behoben, und mögliche Verbesserungen und Features werden iterativ implementiert.
 
### Evaluation

Die Abschlussphase der Bachelorarbeit beinhaltet eine detaillierte Evaluation, in der die Entwicklung und Umsetzung des Projekts analysiert werden. In dieser Analyse werden die erfolgreichen Aspekte der Entwicklung sowie die Vorteile der Verwendung von WebAssembly für Schnittstellen hervorgehoben.

Auftretende Probleme und Herausforderungen im Projekt werden ebenfalls dokumentiert. Die Lösungen für diese Probleme werden erläutert, und falls keine Lösungen gefunden wurden, werden Lösungsansätze und Verbesserungsvorschläge präsentiert.

Links:

Unsere präferierte Runtime: 
* https://github.com/bytecodealliance/wasm-micro-runtime
* https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
* https://github.com/WebAssembly/component-model/tree/main
