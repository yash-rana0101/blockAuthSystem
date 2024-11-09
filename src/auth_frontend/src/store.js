// src/store.js
import { configureStore } from "@reduxjs/toolkit";
import actorsReducer from "./redux/actorsSlice"; // Adjust this path if necessary

const store = configureStore({
  reducer: {
    actors: actorsReducer,
  },
});

export default store;
