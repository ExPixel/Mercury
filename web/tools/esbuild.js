import { pnpPlugin } from '@yarnpkg/esbuild-plugin-pnp';
import { build } from 'esbuild';

build({
    entryPoints: ['src/main.ts'],
    bundle: true,
    outfile: 'dist/out.js',
    plugins: [pnpPlugin()],
});