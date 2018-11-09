import React from "react";
import ReactDom from "react-dom";

const append = (oldArray, newItem) => [...oldArray, newItem];

const Item = props => (
  <div className="fl w-75 bg-light-gray shadow-4 ma2 pa2 ba b--mid-grey">
    <div className="fl w-75 i pa2">{props.name}</div>
    <div className="fl w-25 b pa2">{props.value}</div>
  </div>
);

class EventPipe extends React.Component {
  constructor(props) {
    super(props);
    this.state = { items: [] };
    this.addElement = this.addElement.bind(this);
  }

  addElement() {
    const item = <Item name="Chaos Orb" value="1c" />;
    this.setState({
      items: append(this.state.items, item)
    });
  }

  render() {
    return (
      <div className="fl w-20 ma2 pa1">
        <h1 className="h1">Event Pipe</h1>
        <button className="br2 pa2" onClick={this.addElement}>
          Add an element!
        </button>
        <div className="pt2">
          There are {this.state.items.length} items in the pipe.
        </div>
        <div className="pt3 item-center">{this.state.items}</div>
      </div>
    );
  }
}

class MapDisplay extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      currentMap: props.currentMap,
      previousMap: props.previousMap
    };

    this.updateCurrentMap = this.updateCurrentMap.bind(this);
  }

  updateCurrentMap(newMap) {
    this.setState({
      currentMap: newMap,
      previousMap: this.state.currentMap
    });
  }

  render() {
    return (
      <div className="fl w-50">
        <div>
          Current Map:
          <div className="b f1">{this.state.currentMap}</div>
        </div>
        <div className="pt3">
          Previous Map:
          <div className="b f3 pt1"> {this.state.previousMap} </div>
        </div>
      </div>
    );
  }
}

class Display extends React.Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  render() {
    return (
      <div className="fl w-100 helvetica">
        <EventPipe />
        <MapDisplay
          currentMap="Shaped Cage"
          previousMap="Shaped Underground Sea"
        />
      </div>
    );
  }
}

const mountNode = document.getElementById("mount");
ReactDom.render(<Display />, mountNode);
