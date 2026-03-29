import { useNavigate, useLocation } from 'react-router-dom';
import { TrendingDown, LogIn, LogOut } from 'lucide-react';
import { useAuthStore } from '../store';

export function Navbar() {
  const navigate = useNavigate();
  const location = useLocation();
  const { user, logout } = useAuthStore();


  return (
    <nav className="fixed top-0 w-full z-50 bg-white/80 backdrop-blur-xl shadow-sm h-20">
      <div className="w-full px-6 h-full flex justify-between items-center">
        <div className="flex items-center gap-8">
          <div 
            className="text-xl font-extrabold tracking-tight text-slate-900 flex items-center gap-2 cursor-pointer"
            onClick={() => navigate(user ? '/dashboard' : '/')}
          >
            <TrendingDown className="text-primary w-6 h-6" />
            <span className="font-headline">Toll Optimizer</span>
          </div>
          
          {!user && (
            <div className="hidden md:flex gap-6">
              <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#features">Features</a>
              <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#howto">How To</a>
              <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#pricing">Pricing</a>
              <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#about">About</a>
            </div>
          )}
        </div>

        <div className="flex items-center gap-4">
          {user ? (
            <>
              <span className="text-sm text-slate-600 hidden sm:inline font-medium">{user.email}</span>
              <button
                onClick={() => {
                  logout();
                  navigate('/');
                }}
                className="p-2 text-slate-500 hover:text-error hover:bg-error-container/20 rounded-full transition-all"
                title="Logout"
              >
                <LogOut className="w-5 h-5" />
              </button>
            </>
          ) : (
            <>
              {location.pathname !== '/login' && (
                <>
                  <button 
                    onClick={() => navigate('/login')}
                    className="text-slate-600 hover:text-primary transition-all duration-300 px-4 py-2 flex items-center gap-2 font-medium"
                  >
                    <LogIn className="w-4 h-4" />
                    Log In
                  </button>
                  <button 
                    onClick={() => navigate('/login')}
                    className="bg-primary hover:bg-primary-container text-on-primary px-6 py-2.5 rounded-lg font-semibold transition-all duration-300 active:scale-95 shadow-md"
                  >
                    Get Started
                  </button>
                </>
              )}
              {location.pathname === '/login' && (
                <button className="text-sm font-medium text-slate-500 hover:text-primary transition-colors font-headline">Help</button>
              )}
            </>
          )}
        </div>
      </div>
    </nav>
  );
}