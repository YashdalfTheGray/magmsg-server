import Component from '@magmsg/client/Component';

import './content.scss';

export default class Content extends Component {
  constructor() {
    super('Content');
  }

  public render() {
    return `
      <div class="content flex-column flex-item-expand">

      </div>
    `;
  }
}
