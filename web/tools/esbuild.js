import { pnpPlugin } from '@yarnpkg/esbuild-plugin-pnp';
import EsBuild from 'esbuild';
import Url from 'url';
import Path from 'path';
import Process from 'process';

const __filename = Url.fileURLToPath(import.meta.url);
const __dirname = Path.dirname(__filename);

/**
 * @readonly
 * @enum {number}
 */
const Environment = Object.freeze({
    Production: 0,
    Development: 1,
});

/**
 * @param {Environment} environment 
 * @param {boolean} watch
 */
async function build(environment, watch) {
    const sourcemap = environment !== Environment.Production;
    const minify = environment === Environment.Production;

    EsBuild.build({
        entryPoints: ['src/main.ts'],
        outfile: Path.resolve(__dirname, '../../build', 'main.js'),
        loader: { '.woff': 'file', '.woff2': 'file' },
        plugins: [pnpPlugin()],
        bundle: true,
        sourcemap,
        minify,
        watch,
        logLevel: 'info',
    }).catch(() => process.exit(1));
}

/**
 * @param {string[]} args 
 * @return {Environment}
 */
function getEnvironment(args) {
    if (args.length > 2) {
        const env = args[2].toLowerCase();
        if (env === 'prod' || env === 'production' || env === 'watch:prod' || env === 'watch:production') {
            return Environment.Production;
        } else if (env === 'development' || env === 'dev' || env === 'watch' || env === 'watch:dev' || env === 'watch:development') {
            return Environment.Development;
        }
    }
    return Environment.Production;
}

/**
 * @param {Environment} env 
 * @return {string}
 */
function environmentToString(env) {
    switch (env) {
        case Environment.Development: return 'development';
        case Environment.Production: return 'production';
        default: return 'unknown';
    }
}

/**
 * @param {string[]} args 
 * @return {boolean}
 */
function getIsWatch(args) {
    if (args.length > 2) {
        const env = args[2].toLowerCase();
        return env === 'watch' || env === 'watch:prod' || env === 'watch:production' || env === 'watch:dev' || env === 'watch:development';
    }
    return false;
}

/**
 * @param {string[]} args 
 */
async function main(args) {
    const environment = getEnvironment(args);
    const watch = getIsWatch(args);

    console.log('building...');
    console.log('environment: %s', environmentToString(environment));
    console.log('watch: %s', watch);

    await build(environment, watch);
}
await main(Process.argv);