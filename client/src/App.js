import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import axios from 'axios';
import GMAP from './components/googleMap';

class App extends Component {
  componentDidMount() {
      axios.get('/hello/world')
        .then(({data}) => console.log(' la réponse est ' + data))
        .catch(err => console.log(err));
  }
  render() {
    return (
      <div className="App">
        <GMAP/>
      </div>
    );
  }
}

export default App;
