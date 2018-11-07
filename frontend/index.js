import React from 'react';
import ReactDom from 'react-dom';

const Item = (props) => <div className="measure-narrow"> {props.name} </div>;

class EventPipe extends React.Component {
    constructor(props) {
	super(props);
	this.state  = {items: []};
	this.addElement = this.addElement.bind(this);
    }

    addElement() {
	const item = <Item name="Chaos Orb" />;
	this.setState({
	    items: [...this.state.items, item]
	});
    }

    render() {
	return <div className="fl w-25 r">
	    <h1>Event Pipe :D</h1>
	    <button onClick={this.addElement}> Add an element!</button>
	    <div>There are {this.state.items.length} items in the pipe.</div>
	    <div>{this.state.items}</div>
	</div>;
    }
}

const mountNode = document.getElementById("mount");
ReactDom.render(<EventPipe />, mountNode);
