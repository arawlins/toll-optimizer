import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store';
import { endpoints } from '../lib/api';
import { Loader2, Lock, TrendingDown } from 'lucide-react';
import { Navbar } from '../components/Navbar';

export function Login() {
  const [isRegistering, setIsRegistering] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const { setAuth } = useAuthStore();
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const fn = isRegistering ? endpoints.register : endpoints.login;
      const res = await fn({ email, password });
      setAuth(res.data.token, res.data.user);
      navigate('/dashboard');
    } catch (err: any) {
      setError(err.response?.data || 'An error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-slate-50 text-on-surface min-h-screen flex flex-col font-body">
      {/* Top Navigation Bar */}
      <Navbar />

      {/* Main Content */}
      <main className="flex-grow flex items-center justify-center px-4 pt-20 pb-12">
        <div className="w-full max-w-md">
          <div className="bg-surface-container-lowest shadow-[0px_12px_32px_rgba(25,28,29,0.06)] rounded-xl p-8 md:p-10 border border-slate-100">
            <div className="mb-10 text-center">
              <div className="inline-flex items-center justify-center w-16 h-16 bg-surface-container-low rounded-full mb-6 text-primary">
                <Lock className="w-8 h-8" />
              </div>
              <h1 className="text-2xl font-semibold tracking-tight text-on-surface mb-2 font-headline">
                {isRegistering ? 'Create your account' : 'Sign in to your account'}
              </h1>
              <p className="text-on-surface-variant text-sm">
                Access your optimization dashboard and toll reports.
              </p>
            </div>

            <form className="space-y-6" onSubmit={handleSubmit}>
              <div className="space-y-2">
                <label className="block text-[0.6875rem] font-semibold uppercase tracking-widest text-on-surface-variant px-1" htmlFor="email">
                  Email address
                </label>
                <input
                  id="email"
                  name="email"
                  type="email"
                  required
                  placeholder="name@company.com"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  className="w-full px-4 py-3 bg-surface-container-lowest border-0 border-b-2 border-transparent focus:ring-0 focus:border-primary transition-all text-on-surface placeholder:text-outline-variant/60 shadow-[inset_0_0_0_1px_rgba(115,118,133,0.1)] rounded-lg text-sm"
                />
              </div>

              <div className="space-y-2">
                <div className="flex justify-between items-center px-1">
                  <label className="block text-[0.6875rem] font-semibold uppercase tracking-widest text-on-surface-variant" htmlFor="password">
                    Password
                  </label>
                  {!isRegistering && (
                    <a className="text-[0.6875rem] font-semibold text-primary hover:underline uppercase tracking-widest" href="#">Forgot?</a>
                  )}
                </div>
                <input
                  id="password"
                  name="password"
                  type="password"
                  required
                  placeholder="••••••••"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="w-full px-4 py-3 bg-surface-container-lowest border-0 border-b-2 border-transparent focus:ring-0 focus:border-primary transition-all text-on-surface placeholder:text-outline-variant/60 shadow-[inset_0_0_0_1px_rgba(115,118,133,0.1)] rounded-lg text-sm"
                />
              </div>

              {error && (
                <div className="bg-error-container text-error text-xs p-3 rounded-lg font-medium border border-error/10">
                  {error}
                </div>
              )}

              <button
                type="submit"
                disabled={loading}
                className="w-full bg-primary text-on-primary py-3.5 px-4 rounded-xl font-semibold text-sm tracking-wide shadow-sm hover:opacity-90 active:scale-[0.98] transition-all duration-200 flex items-center justify-center gap-2 disabled:opacity-50"
              >
                {loading && <Loader2 className="w-4 h-4 animate-spin" />}
                {isRegistering ? 'Register' : 'Sign in'}
              </button>

              <div className="relative py-2">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-outline-variant/20"></div>
                </div>
                <div className="relative flex justify-center text-[0.6875rem] uppercase tracking-[0.2em] font-bold">
                  <span className="bg-surface-container-lowest px-4 text-outline">Or</span>
                </div>
              </div>

              <button
                type="button"
                onClick={() => setIsRegistering(!isRegistering)}
                className="w-full bg-white border border-outline-variant/40 text-on-surface py-3.5 px-4 rounded-xl font-semibold text-sm tracking-wide hover:bg-surface-container-low transition-colors duration-200"
              >
                {isRegistering ? 'Sign in existing account' : 'Create new account'}
              </button>
            </form>

            <div className="mt-8 pt-8 border-t border-outline-variant/10 text-center">
              <p className="text-xs text-on-surface-variant leading-relaxed">
                By signing in, you agree to our 
                <a className="text-primary hover:underline font-medium mx-1" href="#">Terms of Service</a> and 
                <a className="text-primary hover:underline font-medium mx-1" href="#">Privacy Policy</a>.
              </p>
            </div>
          </div>
          
          {/* Background Decoration */}
          <div className="fixed -bottom-24 -left-24 w-96 h-96 bg-primary/5 rounded-full blur-3xl -z-10"></div>
          <div className="fixed -top-24 -right-24 w-96 h-96 bg-tertiary/5 rounded-full blur-3xl -z-10"></div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-slate-50 w-full py-8 mt-auto border-t border-slate-100">
        <div className="flex flex-col md:flex-row justify-between items-center px-8 max-w-7xl mx-auto gap-4">
          <div className="flex items-center gap-2">
            <TrendingDown className="text-primary w-5 h-5" />
            <span className="font-headline font-bold text-slate-900">Toll Optimizer</span>
          </div>
          <div className="flex gap-8">
            <a className="text-xs tracking-widest uppercase text-slate-400 hover:text-primary transition-colors font-medium" href="#">Privacy Policy</a>
            <a className="text-xs tracking-widest uppercase text-slate-400 hover:text-primary transition-colors font-medium" href="#">Terms</a>
            <a className="text-xs tracking-widest uppercase text-slate-400 hover:text-primary transition-colors font-medium" href="#">Security</a>
          </div>
          <p className="text-xs tracking-widest uppercase text-slate-400 font-medium">© {new Date().getFullYear()} Toll Optimizer</p>
        </div>
      </footer>
    </div>
  );
}