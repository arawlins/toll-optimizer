import { useState, useRef, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { TrendingDown, LogIn, LogOut, User, ChevronDown, LayoutDashboard } from 'lucide-react';
import { useAuthStore } from '../store';

export function Navbar() {
  const navigate = useNavigate();
  const location = useLocation();
  const { user, logout } = useAuthStore();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsMenuOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <nav className="fixed top-0 w-full z-50 bg-white/80 backdrop-blur-xl shadow-sm h-20">
      <div className="w-full px-6 h-full flex justify-between items-center">
        <div className="flex items-center gap-8">
          <div 
            className="text-xl font-extrabold tracking-tight text-slate-900 flex items-center gap-2 cursor-pointer"
            onClick={() => navigate(user ? '/dashboard' : '/')}
          >
            <TrendingDown className="text-primary w-6 h-6" />
            <span className="font-headline font-black">Toll Optimizer</span>
          </div>
          
          <div className="hidden md:flex gap-6 items-center">
            {!user ? (
              <>
                <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#features">Features</a>
                <button 
                  className={`font-manrope text-sm font-medium tracking-tight transition-colors duration-200 ${location.pathname === '/how-to' ? 'text-primary border-b-2 border-primary pb-1' : 'text-slate-500 hover:text-primary'}`}
                  onClick={() => navigate('/how-to')}
                >
                  How To
                </button>
                <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#pricing">Pricing</a>
                <a className="font-manrope text-sm font-medium tracking-tight text-slate-500 hover:text-primary transition-colors duration-200" href="/#about">About</a>
              </>
            ) : (
              <>
                {location.pathname !== '/dashboard' && (
                  <button 
                    onClick={() => navigate('/dashboard')}
                    className="flex items-center gap-2 text-sm font-bold text-primary hover:bg-primary/5 px-4 py-2 rounded-xl transition-all"
                  >
                    <LayoutDashboard className="w-4 h-4" />
                    Dashboard
                  </button>
                )}
              </>
            )}
          </div>
        </div>

        <div className="flex items-center gap-4">
          {user ? (
            <div className="relative" ref={menuRef}>
              <button
                onClick={() => setIsMenuOpen(!isMenuOpen)}
                className="flex items-center gap-3 pl-4 pr-3 py-2 bg-slate-50 hover:bg-slate-100 border border-slate-200 rounded-full transition-all group"
              >
                <div className="w-7 h-7 bg-primary/10 rounded-full flex items-center justify-center text-primary group-hover:bg-primary/20 transition-colors">
                  <User className="w-4 h-4" />
                </div>
                <span className="text-sm text-slate-700 font-bold max-w-[150px] truncate">{user.email}</span>
                <ChevronDown className={`w-4 h-4 text-slate-400 transition-transform duration-200 ${isMenuOpen ? 'rotate-180' : ''}`} />
              </button>

              {isMenuOpen && (
                <div className="absolute right-0 mt-2 w-56 bg-white rounded-2xl shadow-xl border border-slate-100 py-2 animate-in fade-in zoom-in-95 duration-100 origin-top-right">
                  <div className="px-4 py-3 border-b border-slate-50 mb-1">
                    <p className="text-[10px] font-bold text-slate-400 uppercase tracking-widest mb-0.5">Signed in as</p>
                    <p className="text-xs font-bold text-slate-900 truncate">{user.email}</p>
                  </div>
                  <button
                    onClick={() => {
                      logout();
                      setIsMenuOpen(false);
                      navigate('/');
                    }}
                    className="w-full flex items-center gap-3 px-4 py-2.5 text-sm text-error hover:bg-error-container/10 transition-colors font-bold"
                  >
                    <LogOut className="w-4 h-4" />
                    Logout
                  </button>
                </div>
              )}
            </div>
          ) : (
            <>
              {location.pathname !== '/login' && (
                <>
                  <button 
                    onClick={() => navigate('/login')}
                    className="text-slate-600 hover:text-primary transition-all duration-300 px-4 py-2 flex items-center gap-2 font-bold"
                  >
                    <LogIn className="w-4 h-4" />
                    Log In
                  </button>
                  <button 
                    onClick={() => navigate('/login')}
                    className="bg-primary hover:bg-primary-container text-on-primary px-6 py-2.5 rounded-xl font-bold transition-all duration-300 active:scale-95 shadow-md"
                  >
                    Get Started
                  </button>
                </>
              )}
            </>
          )}
        </div>
      </div>
    </nav>
  );
}
