# webAssembly-bachelor

Exposee:

1. Problemstellung
   
Thema der Bachelorarbeit ist das Erstellen und Umsetzen von Interfaces für Sensoren in WebAssembly. Für die Umsetzung von WebAssembly wird die WebAssembly Micro Runtime (WASMR) verwendet, die eine Vielzahl an Tools und Libraries mit sich bringt um WebAssembly in Projekte einzubinden und umzusetzen. 

Interfaces in WebAssembly können mittels dem Component-model definiert werden. Das Component-model stellt hierfür das Wasm Interface Type (WIT) Format zur Verfügung. Mithilfe vom WIT Format können Packages erstellt werden in denen Typen, Interfaces und Components definiert und beschrieben werden. Diese wiederum können sowohl von anderen Packages importiert als auch wieder exportiert werden. 

2. Wie ich vorgehen werde

Zunächst sollte WASMR korrekt in das Projekt eingebunden werden. Anschließend werden mithilfe vom WIT Format die Interfaces für die Sensoren entsprechend der Anforderungen umgesetzt. 

Die Interfaces sollen auf korrekte Funktionalität getestet werden. Dafür kann z.B ein Gerät mit Arduino gebaut werden mit dem man die Sensoren testet, z.B wie gut ein Lichtsensor dunkle und helle Bereiche erkennt. 


3. Evaluation

Das Projekt soll anschließend evaluiert werden, indem es hierfür die Entwicklung analysiert wird. In der Analyse wird festgehalten was in der Entwicklung gut lief und welche Vorteile die Umsetzung mittels WebAssembly gebracht hat. Auch Probleme und Schwierigkeiten sollen aufgeführt werden, mögliche Verbesserungen und Lösungen können potentiell mit eingebracht werden. 


Links:

Unsere präferierte Runtime: 
* https://github.com/bytecodealliance/wasm-micro-runtime
* https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
* https://github.com/WebAssembly/component-model/tree/main
