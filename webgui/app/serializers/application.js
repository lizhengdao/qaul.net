import JSONSerializer from '@ember-data/serializer/json';
import { pluralize } from 'ember-inflector';
import { underscore } from '@ember/string';

export default class ApplicationSerializer extends JSONSerializer {
  keyForAttribute(attr) {
    return underscore(attr);
  }
  normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {
    // we convert from { user: [ { ...data... } ] } to { ...data... }
    return super.normalizeSingleResponse(store, primaryModelClass, payload[underscore(primaryModelClass.modelName)], id, requestType);
  }
  normalizeArrayResponse(store, primaryModelClass, payload, id, requestType) {
    const primary = payload[underscore(primaryModelClass.modelName)] || payload[underscore(pluralize(primaryModelClass.modelName))] || payload[Object.keys(payload)[0]];

    return super.normalizeArrayResponse(store, primaryModelClass, primary, id, requestType);
  }

  serialize(snapshot, options) {
    let json = {};

    if (options && options.includeId) {
      const id = snapshot.id;
      if (id) {
        json[this.primaryKey] = id;
      }
    }

    const changedAttributes = Object.keys(snapshot.changedAttributes());
    snapshot.eachAttribute((key, attribute) => {
      if(changedAttributes.includes(key)) {
        this.serializeAttribute(snapshot, json, key, attribute);
      }
    });

    snapshot.eachRelationship((key, relationship) => {
      if (relationship.kind === 'belongsTo') {
        this.serializeBelongsTo(snapshot, json, relationship);
      } else if (relationship.kind === 'hasMany') {
        this.serializeHasMany(snapshot, json, relationship);
      }
    });

    console.log(json);
    return json;
  }
}
