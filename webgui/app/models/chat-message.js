import Model, { attr, belongsTo } from '@ember-data/model';

export default class ChatMessage extends Model {
  @attr text;
  @belongsTo('chat-room', { inverse: 'messages' }) room;
}

// content: attr('string'),
// timestamp: attr('date'),

// sender: belongsTo('user'),
