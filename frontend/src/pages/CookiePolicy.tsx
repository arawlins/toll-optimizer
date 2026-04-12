import { useState } from 'react';
import { 
  Info, 
  Cookie, 
  LineChart, 
  Settings,
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';

/**
 * Toll Optimizer - Cookie Policy Screen (Refined)
 * 
 * An editorial-style Cookie Policy page designed with the "Equilibrium Finance" 
 * visual language. Features a clean, single-column layout with split-section 
 * typography and interactive preference toggles.
 */
export function CookiePolicy() {
  const [analyticsEnabled, setAnalyticsEnabled] = useState(true);
  const [functionalEnabled, setFunctionalEnabled] = useState(false);

  return (
    <div className="min-h-screen bg-white font-manrope text-[#1e293b] antialiased">
      {/* Navigation Header */}
      <Navbar />

      <main className="max-w-4xl mx-auto px-6 py-16 md:py-32">
        {/* Header Section */}
        <div className="mb-16">
          <span className="inline-block px-3 py-1 rounded-full bg-blue-50 text-blue-700 text-[10px] font-bold uppercase tracking-wider mb-6">
            Legal Framework
          </span>
          <h1 className="text-5xl md:text-6xl font-extrabold tracking-tight text-slate-900 mb-6 font-headline">
            Cookie Policy
          </h1>
          <div className="flex flex-col md:flex-row justify-between items-start md:items-end gap-4">
            <p className="text-lg text-slate-500 max-w-xl leading-relaxed">
              We believe in radical transparency. This document explains how we use tracking technologies to optimize your financial journey while respecting your digital boundaries.
            </p>
            <div className="text-right">
              <span className="block text-[10px] font-bold uppercase tracking-widest text-slate-400">Last Updated</span>
              <span className="text-sm font-bold text-slate-900">January 15, 2025</span>
            </div>
          </div>
        </div>

        {/* Policy Content Sections */}
        <div className="space-y-24">
          
          {/* Introduction */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2 flex items-center gap-3">
              <Info className="text-blue-600 w-5 h-5" />
              <h2 className="text-xl font-bold text-slate-900 font-headline">Introduction</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-6">
                Toll Optimizer ("we", "us", or "our") uses cookies and similar technologies to provide, protect, and improve our services. This policy provides detailed information about why we use cookies and how you can control them.
              </p>
              <p className="text-slate-600 leading-relaxed">
                By continuing to use our platform, you acknowledge our use of cookies as described in this policy. We ensure that your financial data and browsing habits are handled with the highest level of encryption and privacy.
              </p>
            </div>
          </section>

          {/* What are cookies? */}
          <section className="bg-slate-50 rounded-[32px] p-8 md:p-12 border border-slate-100">
            <div className="flex items-center gap-3 mb-8">
              <Cookie className="text-blue-600 w-6 h-6" />
              <h2 className="text-3xl font-extrabold text-slate-900 font-headline">What are cookies?</h2>
            </div>
            <p className="text-slate-600 leading-relaxed mb-10 max-w-2xl">
              Cookies are small text files that are stored in your web browser that allow Toll Optimizer or a third party to recognize you. Cookies can be used to collect, store and share bits of information about your activities across websites, including on the Toll Optimizer website.
            </p>
            
            <div className="grid md:grid-cols-2 gap-6">
              <div className="bg-white p-6 rounded-2xl border border-slate-100">
                <h3 className="font-bold text-slate-900 mb-2 text-sm">Session Cookies</h3>
                <p className="text-xs text-slate-500 leading-relaxed">
                  These are temporary and expire once you close your browser (or once your session ends).
                </p>
              </div>
              <div className="bg-white p-6 rounded-2xl border border-slate-100">
                <h3 className="font-bold text-slate-900 mb-2 text-sm">Persistent Cookies</h3>
                <p className="text-xs text-slate-500 leading-relaxed">
                  These remain on your hard drive until you erase them or your browser does, depending on the cookie's expiration date.
                </p>
              </div>
            </div>
          </section>

          {/* How we use cookies */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2 flex items-center gap-3">
              <LineChart className="text-blue-600 w-5 h-5" />
              <h2 className="text-xl font-bold text-slate-900 font-headline">How we use cookies</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-8">
                When you access and use the Service, we may place a number of cookies files in your web browser. We use these cookies for the following purposes:
              </p>
              <div className="space-y-6">
                <div>
                  <h4 className="text-sm font-bold text-slate-900 mb-2 flex items-center gap-2">
                    <span className="w-1.5 h-1.5 rounded-full bg-blue-600"></span>
                    Authentication
                  </h4>
                  <p className="text-xs text-slate-500 leading-relaxed">To recognize you when you visit our website and as you navigate our website.</p>
                </div>
                <div>
                  <h4 className="text-sm font-bold text-slate-900 mb-2 flex items-center gap-2">
                    <span className="w-1.5 h-1.5 rounded-full bg-blue-600"></span>
                    Personalization
                  </h4>
                  <p className="text-xs text-slate-500 leading-relaxed">To store information about your preferences and to personalize the website for you.</p>
                </div>
                <div>
                  <h4 className="text-sm font-bold text-slate-900 mb-2 flex items-center gap-2">
                    <span className="w-1.5 h-1.5 rounded-full bg-blue-600"></span>
                    Security
                  </h4>
                  <p className="text-xs text-slate-500 leading-relaxed">As an element of the security measures used to protect user accounts, including preventing fraudulent use of login credentials.</p>
                </div>
              </div>
            </div>
          </section>

          {/* Types of cookies we use */}
          <section>
            <h2 className="text-3xl font-extrabold text-slate-900 mb-12 font-headline">Types of cookies we use</h2>
            <div className="space-y-4">
              
              {/* Necessary Cookies */}
              <div className="p-8 rounded-2xl bg-slate-50/50 border border-slate-100 flex flex-col md:flex-row justify-between items-start md:items-center gap-6">
                <div className="max-w-xl">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="font-bold text-slate-900">Necessary Cookies</h3>
                    <span className="px-2 py-0.5 rounded bg-green-100 text-green-700 text-[8px] font-black uppercase tracking-widest">Always Active</span>
                  </div>
                  <p className="text-sm text-slate-500 leading-relaxed">
                    These cookies are essential for the website to function and cannot be switched off in our systems. They are usually only set in response to actions made by you which amount to a request for services, such as setting your privacy preferences or logging in.
                  </p>
                </div>
              </div>

              {/* Analytics Cookies */}
              <div className="p-8 rounded-2xl bg-white border border-slate-100 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 shadow-sm">
                <div className="max-w-xl">
                  <h3 className="font-bold text-slate-900 mb-2">Analytics Cookies</h3>
                  <p className="text-sm text-slate-500 leading-relaxed">
                    These cookies allow us to count visits and traffic sources so we can measure and improve the performance of our site. They help us to know which pages are the most and least popular and see how visitors move around the site.
                  </p>
                </div>
                <button 
                  onClick={() => setAnalyticsEnabled(!analyticsEnabled)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none ${analyticsEnabled ? 'bg-blue-600' : 'bg-slate-200'}`}
                >
                  <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${analyticsEnabled ? 'translate-x-6' : 'translate-x-1'}`} />
                </button>
              </div>

              {/* Functional Cookies */}
              <div className="p-8 rounded-2xl bg-white border border-slate-100 flex flex-col md:flex-row justify-between items-start md:items-center gap-6 shadow-sm">
                <div className="max-w-xl">
                  <h3 className="font-bold text-slate-900 mb-2">Functional Cookies</h3>
                  <p className="text-sm text-slate-500 leading-relaxed">
                    These cookies enable the website to provide enhanced functionality and personalization. They may be set by us or by third party providers whose services we have added to our pages.
                  </p>
                </div>
                <button 
                  onClick={() => setFunctionalEnabled(!functionalEnabled)}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none ${functionalEnabled ? 'bg-blue-600' : 'bg-slate-200'}`}
                >
                  <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${functionalEnabled ? 'translate-x-6' : 'translate-x-1'}`} />
                </button>
              </div>

            </div>
          </section>

          {/* How to manage cookies */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2 flex items-center gap-3">
              <Settings className="text-blue-600 w-5 h-5" />
              <h2 className="text-xl font-bold text-slate-900 font-headline">How to manage cookies</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-8">
                Most browsers allow you to refuse to accept cookies and to delete cookies. The methods for doing so vary from browser to browser, and from version to version. You can however obtain up-to-date information about blocking and deleting cookies via these links:
              </p>
              <div className="flex flex-wrap gap-2 mb-6">
                {['Chrome', 'Firefox', 'Safari', 'Edge'].map((browser) => (
                  <button key={browser} className="px-4 py-2 rounded-lg bg-slate-100 text-xs font-bold text-slate-600 hover:bg-slate-200 transition-colors">
                    {browser}
                  </button>
                ))}
              </div>
              <p className="text-[10px] text-slate-400 italic">
                Note: Blocking all cookies will have a negative impact upon the usability of many websites. If you block cookies, you will not be able to use all the features on our website.
              </p>
            </div>
          </section>

        </div>
      </main>

      <Footer />
    </div>
  );
}