import Component from '@magmsg/client/Component';
import { IMessage } from '@magmsg/client/customTypes';

import './messagecard.scss';

export interface IMessageCardProps {
  message: IMessage;
}
export default class MessageCard extends Component<IMessageCardProps> {
  constructor(props: IMessageCardProps) {
    super('MessageCard', props);
  }

  render() {
    const {
      message: { messageId, createdAt, content, createdBy },
    } = this.props;

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
