import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import * as React from 'react';
import ReactDOM from 'react-dom/client';
import App from './components/App';

function main() {
    const container = document.querySelector('#root');
    const root = ReactDOM.createRoot(container);
    root.render(React.createElement(App));
}
main();

export { };