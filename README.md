# webAssembly-bachelor

## Exposee:

### Problemstellung

Das Thema dieser Bachelorarbeit befasst sich mit WebAssembly, wobei der Schwerpunkt auf der Entwicklung und Implementierung von Interfaces für Sensoren liegt. Dabei wird die WebAssembly Micro Runtime (WASMR) als Grundlage verwendet, die eine umfangreiche Sammlung von Tools und Bibliotheken bereitstellt, um WebAssembly in verschiedene Projekte zu integrieren und umzusetzen.

Die Interfaces in WebAssembly werden durch das Component-Modell implementiert. Dieses Modell stellt das Wasm Interface Type (WIT) Format zur Verfügung, mit dem Module erstellt werden können. In diesen Modulen werden Typen und Interfaces textuell beschrieben. Die definierten Typen und Schnittstellen können zwischen verschiedenen Modulen exportiert und importiert werden, was die Erstellung von Bibliotheken ermöglicht. Diese Module können dann für verschiedene Programmiersprachen generiert werden.

### Vorgehensweise

Der erste Schritt besteht darin, sich mit dem vorliegenden Projekt und den spezifizierten Anforderungen vertraut zu machen. Falls erforderlich, wird WebAssembly in das Projekt integriert, um eine Grundlage für die Implementierung der Interfaces zu schaffen.

Nach einer erfolgreichen Integration von WebAssembly in das Projekt kann die Entwicklung und Umsetzung der Interfaces für die Sensoren unter Verwendung des WIT-Formats gemäß den spezifischen Anforderungen beginnen.

Die erstellten Interfaces werden praktisch getestet, indem Geräte mit Arduino erstellt werden, die Sensoren enthalten, an denen die definierten Interfaces getestet werden können. Fehler werden behoben, und es werden mögliche Verbesserungen implementiert.
 
### Evaluation

Die Abschlussphase der Bachelorarbeit beinhaltet eine detaillierte Evaluation, in der die Entwicklung und Umsetzung des Projekts analysiert werden. In dieser Analyse werden die erfolgreichen Aspekte der Entwicklung sowie die Vorteile der Verwendung von WebAssembly für Schnittstellen hervorgehoben.

Auftretende Probleme und Herausforderungen im Projekt werden ebenfalls dokumentiert. Die Lösungen für diese Probleme werden erläutert, und falls keine Lösungen gefunden wurden, werden Lösungsansätze und Verbesserungsvorschläge präsentiert.

Links:

Unsere präferierte Runtime: 
* https://github.com/bytecodealliance/wasm-micro-runtime
* https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
* https://github.com/WebAssembly/component-model/tree/main
