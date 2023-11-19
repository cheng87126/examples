import React from 'react';
import ReactDOM from 'react-dom/client';
// import ReactDOM from 'react-dom';
import {
  createBrowserRouter,
  RouterProvider
} from 'react-router-dom'
import './index.css';
import App from './App';
import Tetris from './pages/Tetris'
import reportWebVitals from './reportWebVitals';

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />
  },
  {
    path: '/tetris',
    element: <Tetris />
  }
])

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
/*
ReactDOM.render(
<React.StrictMode>
  <App />
</React.StrictMode>,
document.getElementById('root') as HTMLElement
)
*/

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
