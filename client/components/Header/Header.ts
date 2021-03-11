import Component from '@magmsg/client/Component';

import './header.scss';

export default class Header extends Component {
  constructor() {
    super('Header');
  }

  public render() {
    return `
      <div class="magmsg-header flex-row flex-item-static">
        <div class="title">Message Sender</div>
      </div>
    `;
  }
}
