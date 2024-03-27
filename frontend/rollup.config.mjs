import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

export default {
  input: 'src/plot/plot.js',
  output: {
    file: 'bundle/plot.bundled.js',
    format: 'esm'
  },
  plugins: [
    commonjs(),
    nodeResolve()
  ]
};
