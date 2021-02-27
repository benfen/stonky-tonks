import { configureStore } from '@reduxjs/toolkit';
import counterReducer from '../features/counter/counterSlice';
import priceReducer from '../features/priceTable/priceSlice';

export default configureStore({
  reducer: {
    counter: counterReducer,
    price: priceReducer,
  },
});
