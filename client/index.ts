import App from '@magmsg/client/App';

const appElement = document.querySelector('#app')!;
appElement.classList.add('expand-to-fill-container');
appElement.innerHTML = new App().render();
