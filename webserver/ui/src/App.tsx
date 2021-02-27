import React from 'react';
import { useDispatch } from 'react-redux';
import './App.css';
import { loadAsync } from './features/priceTable/priceSlice';
import { PriceTable } from './features/priceTable/PriceTable';

function App() {
  const dispatch = useDispatch();
  dispatch(loadAsync());
  return (
    <div>
      <PriceTable />
    </div>
  );
}

export default App;
