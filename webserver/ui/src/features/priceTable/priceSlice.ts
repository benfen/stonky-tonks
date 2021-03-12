import { createSlice } from '@reduxjs/toolkit';
import { Dispatch } from 'react';

export interface StockPrice {
  ticker: string;
  name: string;
  price: number;
}

export interface StockTable {
  prices: Array<StockPrice>;
  pageIndex: number;
  pageSize: number;
}

export const priceSlice = createSlice({
  name: 'price',
  initialState: {
    prices: [],
    pageIndex: 0,
    pageSize: 10
  } as StockTable,
  reducers: {
    load: (state, action) => {
      return {
        ...state,
        prices: action.payload.slice(),
      };
    },
    decreasePageIndex: (state, _) => {
      return {
        ...state,
        pageIndex: state.pageIndex - 1
      };
    },
    increasePageIndex: (state, _) => {
      return {
        ...state,
        pageIndex: state.pageIndex + 1
      };
    },
    setPageIndex: (state, action) => {
      return {
        ...state,
        pageIndex: action.payload
      };
    },
    setPageSize: (state, action) => {
      return {
        ...state,
        pageSize: action.payload,
        pageIndex: 0,
      };
    }
  },
});

export const { decreasePageIndex, increasePageIndex, load, setPageIndex, setPageSize } = priceSlice.actions;

export const loadAsync = () => (dispatch: Dispatch<any>) => {
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

export const selectPaginatedPrices = (state: { price: StockTable }): Array<StockPrice> => {
  if (state.price.prices.length === 0) {
    return [];
  }

  const start = state.price.pageIndex * state.price.pageSize;

  return state.price.prices.slice(start, start + state.price.pageSize);
};

export const selectPrices = (state: { price: StockTable }): Array<StockPrice> => {
  return state.price.prices;
};

export const selectPageIndex = (state: { price: StockTable }): number => {
  return state.price.pageIndex;
};

export const selectPageSize = (state: { price: StockTable }): number => {
  return state.price.pageSize;
};

export default priceSlice.reducer;
