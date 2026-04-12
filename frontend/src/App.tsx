import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { Login } from './pages/Login';
import { Dashboard } from './pages/Dashboard';
import { Landing } from './pages/Landing';
import { HowTo } from './pages/HowTo';
import { PrivacyPolicy } from './pages/PrivacyPolicy';
import { ContactUs } from './pages/ContactUs';
import { TermsOfService } from './pages/TermsOfService';
import { CookiePolicy } from './pages/CookiePolicy';
import { AuthGuard } from './components/AuthGuard';

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Landing />} />
          <Route path="/login" element={<Login />} />
          <Route path="/how-to" element={<HowTo />} />
          <Route path="/privacy-policy" element={<PrivacyPolicy />} />
          <Route path="/contact-us" element={<ContactUs />} />
          <Route path="/terms-of-service" element={<TermsOfService />} />
          <Route path="/cookie-policy" element={<CookiePolicy />} />
          <Route
            path="/dashboard"
            element={
              <AuthGuard>
                <Dashboard />
              </AuthGuard>
            }
          />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}

export default App;