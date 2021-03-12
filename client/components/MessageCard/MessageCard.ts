import Component from '@magmsg/client/Component';
import { IMessage } from '@magmsg/client/customTypes';

import './messagecard.scss';

export default class MessageCard extends Component<IMessage> {
  constructor(props: IMessage) {
    super('MessageCard', props);
  }

  render() {
    const { messageId, createdAt, content, createdBy } = this.props;
    return `
      <div class=".message-card">
        <pre>${messageId}</pre>
        <pre>${createdAt}</pre>
        <pre>${content}</pre>
        <pre>${createdBy}</pre>
      </div>
    `;
  }
}
