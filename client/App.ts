import Component from '@magmsg/client/Component';
import Header from './components/Header';

export default class App extends Component {
  public constructor() {
    super();
  }

  render() {
    return `
    <div class="expand-to-fill-container flex-column">
    ${new Header().render()}
  </div>
    `;
  }
}
