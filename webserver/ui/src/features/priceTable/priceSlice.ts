import { createSlice } from '@reduxjs/toolkit';
import { Dispatch } from 'react';

export interface StockPrice {
  ticker: string;
  name: string;
  price: number;
}

export interface StockTable {
  prices: Array<StockPrice>;
}

export const priceSlice = createSlice({
  name: 'price',
  initialState: {
    prices: [],
  } as StockTable,
  reducers: {
    load: (state, action) => {
      console.log(action);
      state.prices = action.payload.slice();
    },
  },
});

export const { load } = priceSlice.actions;

export const loadAsync = () => (dispatch: Dispatch<any>) => {
  console.log('invoked');
  fetch("/prices")
    .then(response => response.json())
    .then((data) => {
      return data.map((datum: any) => {
        return {
          name: datum.name,
          ticker: datum.symbol,
          price: formatPrice(datum.price),
        }
      })
    })
    .then((data) => {
      dispatch(load(data));
    })
    .catch(console.error)
};

function formatPrice(price: number) {
  const priceString = price.toString();
  return `$${priceString.slice(0, priceString.length - 2)}.${priceString.slice(-2)}`;
}

// The function below is called a selector and allows us to select a value from
// the state. Selectors can also be defined inline where they're used instead of
// in the slice file. For example: `useSelector((state) => state.counter.value)`
export const selectPrices = (state: any) => {
  return state.price.prices;
}

export default priceSlice.reducer;
