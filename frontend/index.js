import React from "react";
import ReactDom from "react-dom";

const append = (oldArray, newItem) => [...oldArray, newItem];

const Item = props => (
  <div className="fl w-75 bg-light-gray shadow-4 ma2 pa2">
    <div className="fl w-75 i pa2">{props.name}</div>
    <div className="fr w-25 b pa2 tr">{props.value}</div>
  </div>
);

const EventPipe = props => {
  return (
    <div className="fl w-20 ma2 pa1">
      <h1 className="h1">Item Log</h1>
      <div className="pt3 item-center">{props.items}</div>
    </div>
  );
};

const MapDisplay = props => {
  return (
    <div className="fl w-50">
      <div>
        Current Map:
        <div className="b f1">{props.currentMap}</div>
      </div>
      <div className="pt3">
        Previous Map:
        <div className="b f3 pt1"> {props.previousMap} </div>
      </div>
    </div>
  );
};

class Display extends React.Component {
  constructor(props) {
    super(props);

    this.dispatch = this.dispatch.bind(this);
    window.dispatch = this.dispatch;

    this.state = {
      currentMap: "Shaped Cage",
      previousMap: "",
      droppedItems: []
    };
  }

  render() {
    return (
      <div className="fl w-100 helvetica">
        <EventPipe items={this.state.droppedItems} />
        <MapDisplay
          currentMap={this.state.currentMap}
          previousMap={this.state.previousMap}
        />
      </div>
    );
  }

  dispatch(string) {
    this.setState({
      currentMap: string,
      previousMap: this.state.currentMap,
      droppedItems: append(this.state.droppedItems, <Item name={string} />)
    });
  }
}

const mountNode = document.getElementById("mount");
ReactDom.render(<Display />, mountNode);
