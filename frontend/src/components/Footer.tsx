import { useNavigate } from 'react-router-dom';
import { TrendingDown } from 'lucide-react';

export function Footer() {
  const navigate = useNavigate();
  
  return (
    <footer className="bg-slate-50 w-full py-12 border-t border-slate-200/20">
      <div className="max-w-7xl mx-auto px-6 flex flex-col md:flex-row justify-between items-center gap-8">
        <div 
          className="text-lg font-bold text-slate-900 flex items-center gap-2 cursor-pointer"
          onClick={() => navigate('/')}
        >
          <TrendingDown className="text-primary w-5 h-5" />
          Toll Optimizer
        </div>
        <div className="flex flex-wrap justify-center gap-8">
          <button 
            onClick={() => navigate('/privacy-policy')}
            className="text-slate-500 text-sm hover:text-blue-600 underline-offset-4 hover:underline transition-colors"
          >
            Privacy Policy
          </button>
          <button 
            onClick={() => navigate('/terms-of-service')}
            className="text-slate-500 text-sm hover:text-blue-600 underline-offset-4 hover:underline transition-colors"
          >
            Terms of Service
          </button>
          <button 
            onClick={() => navigate('/contact-us')}
            className="text-slate-500 text-sm hover:text-blue-600 underline-offset-4 hover:underline transition-colors"
          >
            Contact Us
          </button>
          <button 
            onClick={() => navigate('/cookie-policy')}
            className="text-slate-500 text-sm hover:text-blue-600 underline-offset-4 hover:underline transition-colors"
          >
            Cookie Policy
          </button>
        </div>
        <div className="text-slate-500 text-sm font-medium">
          © {new Date().getFullYear()} Toll Optimizer. All rights reserved.
        </div>
      </div>
    </footer>
  );
}