import React from 'react';
import ReactDom from 'react-dom';

const mountNode = document.getElementById("mount");

class Item extends React.Component {
    render() {
	return <div id="test">
	    Hello {this.props.name}!
	</div>;
    }
}

ReactDom.render(
	<Item name="Meee"/>,
        mountNode);
