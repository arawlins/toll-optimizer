import {
  TrendingDown,
  Wallet,
  FileText,
  ShieldCheck,
  CheckCircle2,
  Lock
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';
import { useState, useEffect } from 'react';
import { endpoints, type PricingResponse } from '../lib/api';
import { Clock } from 'lucide-react';

import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store';

export function Landing() {
  const navigate = useNavigate();
  const { user } = useAuthStore();
  const [pricing, setPricing] = useState<PricingResponse | null>(null);

  useEffect(() => {
    const fetchPricing = async () => {
      try {
        const now = new Date();
        const date = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
        const time = now.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', hour12: true });
        
        const response = await endpoints.pricing({ date, time });
        setPricing(response.data);
      } catch (error) {
        console.error('Failed to fetch pricing:', error);
      }
    };
    fetchPricing();
  }, []);

  return (
    <div className="bg-surface text-on-surface antialiased font-body min-h-screen">
      {/* Top Navigation Bar */}
      <Navbar />

      <main>
        {/* Live Pricing Section */}
        {pricing && (
          <section className="pt-24 pb-12 bg-white border-b border-outline-variant/20">
            <div className="max-w-7xl mx-auto px-6">
              <div className="bg-surface-container-lowest p-8 rounded-2xl shadow-sm border border-primary/20 relative overflow-hidden">
                <div className="absolute top-0 right-0 w-64 h-64 bg-primary/5 rounded-full blur-3xl -mr-20 -mt-20"></div>
                
                <div className="relative z-10 flex flex-col md:flex-row items-center gap-8 justify-between">
                  <div className="flex-1 w-full text-center md:text-left">
                    <div className="flex items-center justify-center md:justify-start gap-3 mb-4">
                      <div className="bg-primary/10 p-3 rounded-full text-primary">
                        <Clock className="w-6 h-6" />
                      </div>
                      <h2 className="text-2xl md:text-3xl font-bold font-headline">Heading out now?</h2>
                    </div>
                    <p className="text-xl font-medium text-slate-700 mb-2">
                      Leave <span className="font-bold text-primary">{pricing.next.average_wb > pricing.current.average_wb ? `before ${pricing.next.timeslot}` : `after ${pricing.next.timeslot}`}</span> to save money
                    </p>
                    <p className="text-sm text-slate-500">Based on live 407 ETR average rates for light vehicles on {pricing.day_type}s.</p>
                  </div>
                  
                  <div className="flex-1 w-full flex flex-col sm:flex-row gap-4 md:gap-8 justify-center md:justify-end items-center">
                    <div className={`p-5 rounded-xl border w-full sm:max-w-[260px] shadow-sm ${pricing.current.average_wb < pricing.next.average_wb ? 'bg-green-50 border-green-200' : 'bg-surface border-slate-100'}`}>
                      <p className={`text-sm font-bold uppercase tracking-wider mb-3 text-center ${pricing.current.average_wb < pricing.next.average_wb ? 'text-green-600' : 'text-slate-400'}`}>
                        Current Timeslot
                        <span className="block opacity-70">({pricing.current.timeslot})</span>
                      </p>
                      <div className="flex justify-between items-center mb-2">
                        <span className="text-slate-600 font-medium">Eastbound</span>
                        <span className="font-bold">{pricing.current.average_eb.toFixed(2)}¢/km</span>
                      </div>
                      <div className="flex justify-between items-center">
                        <span className="text-slate-600 font-medium">Westbound</span>
                        <span className="font-bold">{pricing.current.average_wb.toFixed(2)}¢/km</span>
                      </div>
                    </div>
                    
                    <div className={`p-5 rounded-xl border w-full sm:max-w-[260px] shadow-sm ${pricing.next.average_wb < pricing.current.average_wb ? 'bg-green-50 border-green-200' : 'bg-surface border-slate-100'}`}>
                      <p className={`text-sm font-bold uppercase tracking-wider mb-3 text-center ${pricing.next.average_wb < pricing.current.average_wb ? 'text-green-600' : 'text-slate-400'}`}>
                        Next Timeslot
                        <span className="block opacity-70">({pricing.next.timeslot})</span>
                      </p>
                      <div className="flex justify-between items-center mb-2">
                        <span className="text-slate-600 font-medium">Eastbound</span>
                        <span className="font-bold">{pricing.next.average_eb.toFixed(2)}¢/km</span>
                      </div>
                      <div className="flex justify-between items-center">
                        <span className="text-slate-600 font-medium">Westbound</span>
                        <span className="font-bold">{pricing.next.average_wb.toFixed(2)}¢/km</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </section>
        )}

        {/* Hero Section */}
        <section className="relative pt-12 pb-20 md:pt-24 md:pb-32 overflow-hidden bg-surface">
          <div className="max-w-7xl mx-auto px-6 grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
            <div className="z-10">
              <h1 className="text-5xl md:text-7xl font-extrabold tracking-tight text-on-surface mb-6 leading-[1.1] font-headline">
                Stop Overpaying for the <span className="text-primary">407 ETR</span>
              </h1>
              <p className="text-lg md:text-xl text-on-surface-variant mb-10 max-w-lg leading-relaxed">
                Intelligent toll analysis meets precision auditing. Our algorithms dissect your statements to find hidden errors and optimization opportunities in seconds.
              </p>
              <div className="flex flex-wrap gap-4">
                <button
                  onClick={() => navigate(user ? '/dashboard' : '/login')}
                  className="bg-gradient-to-br from-primary to-primary-container text-white px-8 py-4 rounded-xl font-bold text-lg shadow-lg hover:shadow-xl transition-all duration-300 active:scale-95"
                >
                  Get Started Now
                </button>
                <button 
                  onClick={() => navigate('/how-to')}
                  className="bg-surface-container-high text-primary px-8 py-4 rounded-xl font-bold text-lg hover:bg-surface-container-highest transition-all duration-300"
                >
                  How To
                </button>
              </div>
            </div>
            <div className="relative">
              <div className="absolute -top-20 -right-20 w-96 h-96 bg-primary/5 rounded-full blur-3xl"></div>
              <div className="relative bg-surface-container-lowest p-4 rounded-xl shadow-2xl transform rotate-1 hover:rotate-0 transition-transform duration-500">
                <div className="bg-slate-100 rounded-lg aspect-video flex items-center justify-center overflow-hidden">
                  <div className="text-slate-400 font-bold">Dashboard Mockup</div>
                </div>
                {/* Floating Data Pill */}
                <div className="absolute -bottom-6 -left-6 bg-white p-4 rounded-xl shadow-xl flex items-center gap-3 border border-outline-variant/20">
                  <div className="bg-green-100 p-2 rounded-full text-green-600">
                    <TrendingDown className="w-5 h-5" />
                  </div>
                  <div>
                    <p className="text-xs text-slate-500 font-medium">Avg. Savings</p>
                    <p className="text-lg font-bold text-slate-900">$124.50 /mo</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section id="features" className="py-24 bg-surface-container-low">
          <div className="max-w-7xl mx-auto px-6">
            <div className="text-center mb-20">
              <h2 className="text-3xl md:text-5xl font-bold mb-4 font-headline">Precision Auditing for Peace of Mind</h2>
              <div className="h-1.5 w-24 bg-primary mx-auto rounded-full"></div>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
              {/* Feature 1 */}
              <div className="bg-surface-container-lowest p-8 rounded-xl hover:translate-y-[-8px] transition-all duration-300 group shadow-sm border border-slate-100">
                <div className="w-14 h-14 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary transition-colors duration-300">
                  <Wallet className="text-primary group-hover:text-white w-7 h-7" />
                </div>
                <h3 className="text-2xl font-bold mb-4 font-headline">Save Money</h3>
                <p className="text-on-surface-variant leading-relaxed">Our proprietary algorithms analyze your historical usage and identify cheaper alternatives.</p>
              </div>
              {/* Feature 2 */}
              <div className="bg-surface-container-lowest p-8 rounded-xl hover:translate-y-[-8px] transition-all duration-300 group shadow-sm border border-slate-100">
                <div className="w-14 h-14 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary transition-colors duration-300">
                  <FileText className="text-primary group-hover:text-white w-7 h-7" />
                </div>
                <h3 className="text-2xl font-bold mb-4 font-headline">Simple Upload</h3>
                <p className="text-on-surface-variant leading-relaxed">No manual entry required. Simply drag and drop your CSV statements and watch as our optimizer extracts and analyzes data in under 30 seconds.</p>
              </div>
              {/* Feature 3 */}
              <div className="bg-surface-container-lowest p-8 rounded-xl hover:translate-y-[-8px] transition-all duration-300 group shadow-sm border border-slate-100">
                <div className="w-14 h-14 bg-primary/10 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary transition-colors duration-300">
                  <ShieldCheck className="text-primary group-hover:text-white w-7 h-7" />
                </div>
                <h3 className="text-2xl font-bold mb-4 font-headline">Privacy First</h3>
                <p className="text-on-surface-variant leading-relaxed">Bank-grade AES-256 encryption. We don't store your personal travel history permanently; we analyze, optimize, and clear the slate.</p>
              </div>
            </div>
          </div>
        </section>

        {/* Transparency Statistics Section */}
        <section className="py-24 bg-surface">
          <div className="max-w-7xl mx-auto px-6">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
              <div>
                <h2 className="text-4xl font-bold mb-6 font-headline">Transparency is our Foundation</h2>
                <p className="text-lg text-on-surface-variant mb-8 leading-relaxed">
                  In the world of toll auditing, precision and trust are everything. We've built our system to be a "Zero-Knowledge" platform where your data security is the ultimate priority.
                </p>
                <ul className="space-y-4">
                  <li className="flex items-start gap-3 font-medium text-slate-700">
                    <CheckCircle2 className="text-primary mt-1 w-5 h-5 flex-shrink-0" />
                    <span>Fully automated analysis eliminates human error and bias.</span>
                  </li>
                </ul>
              </div>
              <div className="grid grid-cols-2 gap-6">
                <div className="bg-surface-container-low p-10 rounded-xl text-center shadow-sm">
                  <div className="text-6xl font-extrabold text-primary mb-2 font-headline">0</div>
                  <div className="text-sm font-bold uppercase tracking-widest text-on-surface-variant">Trips Stored</div>
                  <p className="text-xs mt-4 text-slate-500">We delete logs after analysis</p>
                </div>
                <div className="bg-primary p-10 rounded-xl text-center text-on-primary shadow-xl">
                  <div className="text-6xl font-extrabold mb-2 font-headline">100%</div>
                  <div className="text-sm font-bold uppercase tracking-widest opacity-80">Audit Accuracy</div>
                  <p className="text-xs mt-4 opacity-70">Cross-verified algorithms</p>
                </div>
                <div className="col-span-2 bg-surface-container-lowest p-8 rounded-xl flex items-center justify-between border border-outline-variant/20 shadow-sm">
                  <div className="flex items-center gap-6">
                    <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center text-green-600">
                      <CheckCircle2 className="w-9 h-9" />
                    </div>
                    <div>
                      <h4 className="text-xl font-bold font-headline">Security Verified</h4>
                      <p className="text-on-surface-variant">Encrypted end-to-end</p>
                    </div>
                  </div>
                  <Lock className="text-slate-300 w-10 h-10 hidden md:block" />
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* CTA Section */}
        <section className="py-24 relative overflow-hidden bg-primary-container">
          <div className="absolute top-0 left-0 w-full h-full opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(circle at 2px 2px, white 1px, transparent 0)', backgroundSize: '40px 40px' }}></div>
          <div className="max-w-4xl mx-auto px-6 text-center text-on-primary relative z-10">
            <h2 className="text-4xl md:text-5xl font-extrabold mb-8 font-headline leading-tight">Ready to start saving on your commute?</h2>
            <p className="text-xl mb-12 opacity-90 font-medium">Join 5,000+ commuters who have optimized their 407 ETR bills this month.</p>
            <button
              onClick={() => navigate('/login')}
              className="bg-white text-primary hover:bg-surface-bright px-12 py-5 rounded-xl font-bold text-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-105 active:scale-95"
            >
              Analyze My Statement Free
            </button>
          </div>
        </section>
      </main>

      <Footer />
    </div>
  );
}