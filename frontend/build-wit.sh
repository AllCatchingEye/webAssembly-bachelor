npx jco componentize bundle/plot.bundled.js \
	--wit src/plot/plot.wit \
	--world-name plot \
	--out dist/plot.wasm

# npx jco componentize src/http/http.js \
# 	--wit src/http/http.wit \
# 	--world-name http \
# 	--out src/http/http.wasm \
# 	--enable-stdout

# npx jco transpile src/http/http.wasm \
# 	--map wasi:http/*@0.2.0=@bytecodealliance/preview2-shim/http#* \
# 	--map local:http/httpHandle=../http.js \
# 	-o src/http/out/

npx jco transpile dist/plot.wasm \
	--stub \
	-o dist \
	--map local:plot/plot-functions=../../dist/plot.js \
	--map 'wasi-*=@bytecodealliance/preview2-shim/*'
