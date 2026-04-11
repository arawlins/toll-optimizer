import { 
  LogIn, 
  Download, 
  CloudUpload, 
  LineChart, 
  Plus, 
  FileText,
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';

import { useNavigate } from 'react-router-dom';

export function HowTo() {
  const navigate = useNavigate();

  return (
    <div className="bg-background text-on-surface antialiased font-body min-h-screen">
      {/* Top Navigation Bar */}
      <Navbar />

      <main className="pt-32 pb-24 px-6 max-w-7xl mx-auto">
        {/* Hero Section */}
        <header className="mb-24 text-center">
          <span className="inline-block px-4 py-1.5 mb-6 text-sm font-semibold tracking-wider uppercase bg-secondary-container text-on-secondary-container rounded-full font-label">
            Step-by-Step Guide
          </span>
          <h1 className="text-5xl md:text-7xl font-extrabold font-headline tracking-tight text-on-surface mb-8 max-w-4xl mx-auto leading-tight">
            How to Optimize Your 407 ETR Bill
          </h1>
          <p className="text-xl text-on-surface-variant max-w-2xl mx-auto leading-relaxed">
            Master your transportation costs. Follow these four precise steps to transform your raw invoice data into actionable savings strategies.
          </p>
        </header>

        {/* Bento Grid Process Section */}
        <div className="grid grid-cols-1 md:grid-cols-12 gap-8 mb-24">
          {/* Step 1: Login (Large Card) */}
          <div className="md:col-span-7 bg-surface-container-lowest rounded-3xl p-10 shadow-[0_32px_64px_-12px_rgba(25,28,29,0.06)] flex flex-col justify-between overflow-hidden relative group">
            <div className="relative z-10">
              <div className="w-14 h-14 rounded-2xl bg-primary/10 flex items-center justify-center mb-6">
                <LogIn className="text-primary w-8 h-8" />
              </div>
              <h3 className="text-3xl font-bold font-headline mb-4">Step 1: Secure Access</h3>
              <p className="text-on-surface-variant text-lg leading-relaxed max-w-md">
                Begin by logging in to your official 407 ETR account. Ensure you have administrative access to view detailed trip histories.
              </p>
            </div>
            <div className="mt-12 flex justify-end">
              <div className="w-full h-48 rounded-xl bg-surface-container-low overflow-hidden flex items-center justify-center border border-outline-variant/20">
                <div className="text-on-surface-variant/40 font-bold text-xl uppercase tracking-widest">Login Interface Mockup</div>
              </div>
            </div>
            <div className="absolute -top-4 -right-4 text-9xl font-black text-surface-container-high opacity-20 pointer-events-none">01</div>
          </div>

          {/* Step 2: Download (Tall Card) */}
          <div className="md:col-span-5 bg-surface-container-low rounded-3xl p-10 flex flex-col justify-between relative overflow-hidden">
            <div>
              <div className="w-14 h-14 rounded-2xl bg-primary/10 flex items-center justify-center mb-6">
                <Download className="text-primary w-8 h-8" />
              </div>
              <h3 className="text-3xl font-bold font-headline mb-4">Step 2: Export Data</h3>
              <p className="text-on-surface-variant text-lg leading-relaxed">
                Navigate to 'My Invoices'. Select your desired billing period and download your statement specifically as a <strong>CSV file</strong>.
              </p>
            </div>
            <div className="p-6 bg-surface-container-lowest rounded-2xl mt-8 border-b-2 border-outline-variant/20 relative z-10">
              <div className="flex items-center gap-4 mb-3 text-on-surface">
                <FileText className="text-tertiary w-6 h-6" />
                <span className="text-sm font-semibold font-label">billing_july_2024.csv</span>
              </div>
              <div className="w-full bg-surface-container-highest h-2 rounded-full overflow-hidden">
                <div className="bg-primary h-full w-full"></div>
              </div>
            </div>
            <div className="absolute -top-4 -right-4 text-9xl font-black text-surface-container-highest opacity-30 pointer-events-none">02</div>
          </div>

          {/* Step 3: Upload (Square-ish Card) */}
          <div className="md:col-span-5 bg-primary rounded-3xl p-10 text-on-primary flex flex-col justify-between relative overflow-hidden">
            <div className="relative z-10">
              <div className="w-14 h-14 rounded-2xl bg-white/20 flex items-center justify-center mb-6">
                <CloudUpload className="text-white w-8 h-8" />
              </div>
              <h3 className="text-3xl font-bold font-headline mb-4">Step 3: Sync to Optimizer</h3>
              <p className="text-blue-100 text-lg leading-relaxed">
                Simply drag and drop your CSV file into the Toll Optimizer dashboard. Our AI instantly parses your routes and vehicle classes.
              </p>
            </div>
            <div className="mt-8 relative z-10">
              <button 
                onClick={() => navigate('/login')}
                className="w-full py-4 bg-white text-primary rounded-xl font-bold flex items-center justify-center gap-2 hover:bg-blue-50 transition-colors"
              >
                Select File <Plus className="w-5 h-5" />
              </button>
            </div>
            <div className="absolute -top-4 -right-4 text-9xl font-black text-white/10 pointer-events-none">03</div>
          </div>

          {/* Step 4: Review (Wide Card) */}
          <div className="md:col-span-7 bg-surface-container-lowest rounded-3xl p-10 shadow-[0_32px_64px_-12px_rgba(25,28,29,0.06)] flex flex-col md:flex-row gap-8 relative overflow-hidden">
            <div className="flex-1">
              <div className="w-14 h-14 rounded-2xl bg-tertiary/10 flex items-center justify-center mb-6">
                <LineChart className="text-tertiary w-8 h-8" />
              </div>
              <h3 className="text-3xl font-bold font-headline mb-4">Step 4: Strategic Review</h3>
              <p className="text-on-surface-variant text-lg leading-relaxed">
                Review your savings report. We highlight peak-time alternatives and vehicle class corrections that can save you up to 30% monthly.
              </p>
            </div>
            <div className="flex-1 bg-surface-container-low rounded-2xl p-6">
              <div className="space-y-4">
                <div className="flex justify-between items-end">
                  <span className="text-sm font-label text-on-surface-variant">Estimated Savings</span>
                  <span className="text-2xl font-bold text-primary font-headline">$482.50</span>
                </div>
                <div className="h-24 flex items-end gap-2">
                  <div className="flex-1 bg-primary/20 h-1/2 rounded-t-sm"></div>
                  <div className="flex-1 bg-primary/20 h-2/3 rounded-t-sm"></div>
                  <div className="flex-1 bg-primary h-full rounded-t-sm"></div>
                  <div className="flex-1 bg-primary/20 h-3/4 rounded-t-sm"></div>
                </div>
              </div>
            </div>
            <div className="absolute -top-4 -right-4 text-9xl font-black text-surface-container-high opacity-20 pointer-events-none">04</div>
          </div>
        </div>

        {/* Call to Action Section */}
        <section className="bg-surface-container-low rounded-[2rem] p-12 md:p-20 text-center relative overflow-hidden border border-outline-variant/10">
          <div className="relative z-10">
            <h2 className="text-4xl md:text-5xl font-extrabold font-headline mb-6 text-on-surface">Ready to start saving?</h2>
            <p className="text-xl text-on-surface-variant mb-10 max-w-xl mx-auto">
              Join over 5,000 businesses and commuters who have optimized their toll spending with our editorial-grade insights.
            </p>
            <button 
              onClick={() => navigate('/login')}
              className="bg-gradient-to-br from-primary to-primary-container text-on-primary px-10 py-5 rounded-xl text-lg font-bold shadow-xl hover:scale-105 active:scale-95 transition-all"
            >
              Get Started Now
            </button>
          </div>
          {/* Decorative Background Element */}
          <div className="absolute top-0 right-0 w-64 h-64 bg-primary/5 rounded-full -translate-y-1/2 translate-x-1/2 blur-3xl"></div>
          <div className="absolute bottom-0 left-0 w-96 h-96 bg-tertiary/5 rounded-full translate-y-1/2 -translate-x-1/2 blur-3xl"></div>
        </section>
      </main>

      <Footer />
    </div>
  );
}
