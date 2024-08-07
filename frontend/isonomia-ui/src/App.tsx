import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { Home } from './pages/Home';
import { Vote } from './pages/Vote';
import { Results } from './pages/Results';

function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Home />} />
        <Route path="vote" element={<Vote />} />
        <Route path="results" element={<Results />} />
      </Route>
    </Routes>
  );
}

export default App;
