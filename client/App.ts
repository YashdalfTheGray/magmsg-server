import Component from '@magmsg/client/Component';
import Header from '@magmsg/client/components/Header';
import Content from '@magmsg/client/components/Content';

export default class App extends Component {
  public constructor() {
    super();
  }

  render() {
    return `
    <div class="expand-to-fill-container flex-column">
      ${new Header().render()}
      ${new Content().render()}
    </div>
    `;
  }
}
