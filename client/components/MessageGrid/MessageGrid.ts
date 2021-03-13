import Component from '@magmsg/client/Component';
import { IMessage } from '@magmsg/client/customTypes';

import './messagegrid.scss';

export interface IMessageGridProps {
  messages: IMessage[];
}

export default class MessageGrid extends Component<IMessageGridProps> {
  constructor(props: IMessageGridProps) {
    super('MessageGrid');
  }

  render() {
    return `
      <div class="message-grid">
        <h2>The messages go here</h2>
      </div>
    `;
  }
}
