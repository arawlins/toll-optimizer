import { useNavigate } from 'react-router-dom';
import { 
  ShieldCheck, 
  FileText,
  ArrowRight,
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';

/**
 * Toll Optimizer - Privacy Policy Screen
 * 
 * This component represents the Privacy Policy page for the Toll Optimizer application.
 * It features a clean, editorial layout using Tailwind CSS and the "Equilibrium Finance"
 * design aesthetic.
 */
export function PrivacyPolicy() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen bg-white font-manrope text-[#1e293b] antialiased">
      {/* Navigation Header */}
      <Navbar />

      <main className="max-w-4xl mx-auto px-6 py-16 md:py-32">
        {/* Header Section */}
        <div className="mb-16">
          <span className="inline-block px-3 py-1 rounded-full bg-blue-50 text-blue-700 text-[10px] font-bold uppercase tracking-wider mb-6">
            Legal Documentation
          </span>
          <h1 className="text-5xl md:text-6xl font-extrabold tracking-tight text-slate-900 mb-6 font-headline">
            Privacy Policy
          </h1>
          <p className="text-lg text-slate-500 max-w-2xl leading-relaxed">
            Last updated: October 24, 2025. This policy describes how Toll Optimizer handles your data with the precision and security your business requires.
          </p>
        </div>

        {/* Content Sections */}
        <div className="space-y-20">
          
          {/* Data We Collect */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <h2 className="text-xl font-bold text-blue-700 font-headline uppercase tracking-tight">Data We Collect</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-8">
                Toll Optimizer is built on the principle of data minimization. We only collect the technical information necessary to optimize your logistics operations.
              </p>
              
              <div className="bg-slate-50 rounded-2xl p-8 border border-slate-100">
                <div className="flex items-start gap-4 mb-4">
                  <div className="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center text-blue-600 shrink-0">
                    <FileText className="w-5 h-5" />
                  </div>
                  <div>
                    <h3 className="font-bold text-slate-900 mb-1">CSV Statement Processing</h3>
                    <p className="text-sm text-slate-500 leading-relaxed">
                      We exclusively process CSV statements from 407 ETR. This includes trip dates, entry/exit points, and vehicle class information required for cost calculation.
                    </p>
                  </div>
                </div>
                <div className="mt-6 ml-14 p-4 bg-red-50 rounded-xl border border-red-100">
                  <p className="text-xs font-bold text-red-900 mb-1">Important Disclaimer:</p>
                  <p className="text-xs text-red-800">
                    Toll Optimizer is an independent logistics tool and is not affiliated with, endorsed by, or partnered with 407 ETR.
                  </p>
                </div>
              </div>
            </div>
          </section>

          {/* How We Use Your Data */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <h2 className="text-xl font-bold text-blue-700 font-headline uppercase tracking-tight">How We Use Your Data</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-6">
                Your data is used solely to generate optimization reports and identify cost-saving opportunities in your toll usage.
              </p>
              <ul className="space-y-4">
                <li className="flex gap-3 text-slate-600">
                  <span className="w-1.5 h-1.5 rounded-full bg-blue-600 mt-2 shrink-0"></span>
                  <span>To calculate alternative route savings based on your historical usage patterns.</span>
                </li>
                <li className="flex gap-3 text-slate-600">
                  <span className="w-1.5 h-1.5 rounded-full bg-blue-600 mt-2 shrink-0"></span>
                  <span>To improve our internal algorithms for more accurate toll prediction.</span>
                </li>
              </ul>
            </div>
          </section>

          {/* Data Retention & Security */}
          <section className="bg-slate-50 rounded-[32px] p-8 md:p-12 border border-slate-100">
            <div className="grid md:grid-cols-2 gap-12 items-center">
              <div>
                <h2 className="text-3xl font-extrabold text-slate-900 mb-6 font-headline">Data Retention & Security</h2>
                <p className="text-slate-500 leading-relaxed mb-8">
                  We believe your data should not live on our servers a minute longer than necessary. Our systems are configured for ephemeral processing.
                </p>
                
                <div className="bg-white rounded-2xl p-6 shadow-sm border border-slate-100 max-w-xs">
                  <div className="flex items-center gap-3 mb-3">
                    <ShieldCheck className="text-blue-600 w-5 h-5" />
                    <span className="text-[10px] font-black uppercase tracking-widest text-slate-400">AES-256</span>
                  </div>
                  <h3 className="font-bold text-slate-900 text-xs mb-2 uppercase tracking-tight">Encryption</h3>
                  <p className="text-[10px] text-slate-500 leading-relaxed">
                    Data is encrypted both at rest and in transit using industry-standard protocols.
                  </p>
                </div>
              </div>
              <div className="relative aspect-square">
                <img 
                  src="https://images.unsplash.com/photo-1550751827-4bd374c3f58b?q=80&w=800&auto=format&fit=crop" 
                  alt="Abstract security representation" 
                  className="rounded-3xl object-cover w-full h-full shadow-2xl"
                />
              </div>
            </div>
          </section>

          {/* Information Sharing */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <h2 className="text-xl font-bold text-blue-700 font-headline uppercase tracking-tight">Information Sharing</h2>
            </div>
            <div className="md:col-span-3 text-slate-600 leading-relaxed">
              We do not sell, trade, or rent your personal or business data to third parties. We only share information with service providers (such as cloud hosting) who assist us in operating our platform, provided those parties agree to keep this information confidential.
            </div>
          </section>

          {/* Your Choices */}
          <section className="grid md:grid-cols-5 gap-8 items-start">
            <div className="md:col-span-2">
              <h2 className="text-xl font-bold text-blue-700 font-headline uppercase tracking-tight">Your Choices</h2>
            </div>
            <div className="md:col-span-3">
              <p className="text-slate-600 leading-relaxed mb-8">
                You maintain full control over your information at all times.
              </p>
              <div className="grid md:grid-cols-2 gap-4">
                <div className="p-6 rounded-2xl bg-slate-50 border border-slate-100">
                  <h3 className="font-bold text-slate-900 mb-2 text-sm">Right to Delete</h3>
                  <p className="text-xs text-slate-500 leading-relaxed">
                    Request immediate deletion of all account data.
                  </p>
                </div>
                <div className="p-6 rounded-2xl bg-slate-50 border border-slate-100">
                  <h3 className="font-bold text-slate-900 mb-2 text-sm">Right to Access</h3>
                  <p className="text-xs text-slate-500 leading-relaxed">
                    Download a machine-readable copy of all data currently held in your profile.
                  </p>
                </div>
              </div>
            </div>
          </section>
        </div>

        {/* Call to Action */}
        <div className="mt-24 p-12 bg-blue-600 rounded-[40px] text-center text-white shadow-xl shadow-blue-200">
          <h2 className="text-3xl font-extrabold mb-4 font-headline">Have questions about your privacy?</h2>
          <p className="text-blue-100 mb-8 max-w-lg mx-auto opacity-90">
            Our security team is ready to provide the transparency you need.
          </p>
          <button 
            onClick={() => navigate('/contact-us')}
            className="bg-white text-blue-600 px-8 py-3.5 rounded-xl font-bold hover:bg-slate-50 transition-all flex items-center gap-2 mx-auto active:scale-95 shadow-lg"
          >
            Contact Us <ArrowRight className="w-5 h-5" />
          </button>
        </div>
      </main>

      <Footer />
    </div>
  );
}