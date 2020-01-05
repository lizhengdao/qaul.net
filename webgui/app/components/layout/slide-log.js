import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";


export default class SlideLog extends Component {

  @action
  slideOut() {
    this.args.slideLogOut();
  }
}
