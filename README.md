# webAssembly-bachelor

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
